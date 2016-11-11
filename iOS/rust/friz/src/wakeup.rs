pub trait WakeupSender : Drop {
  fn wakeup(&self);
}

pub struct WakeupReceiver<'a> {
  callback: &'a mut FnMut(),
  #[allow(dead_code)]
  impl_ptr: Box<Drop>
}

impl<'a> WakeupReceiver<'a> {
  fn invoke(&mut self) {
    (self.callback)();
  }
}

pub trait WakeupBuilder {
  fn create_wakeup_channel<'a>(&mut self, callback: &'a mut FnMut()) -> (Box<WakeupSender>, Box<WakeupReceiver<'a>>);
}

#[cfg(any(target_os="macos", target_os="ios"))]
pub fn create_wakeup_builder() -> apple::WakeupBuilder {
  apple::WakeupBuilder::new()
}

#[cfg(any(target_os="macos", target_os="ios"))]
mod apple {

  extern crate std;

  use std::os::raw::c_void;

  #[allow(non_camel_case_types)]
  #[allow(dead_code)]
  mod ffi {

    use std::os::raw::c_void;

    pub type dispatch_object_t = *const c_void;
    pub type dispatch_queue_t = *const c_void;
    pub type dispatch_source_t = *const c_void;
    pub type dispatch_source_type_t = *const c_void;

    #[link(name = "dispatch")]
    extern "C" {
      pub fn dispatch_get_main_queue() -> dispatch_queue_t;


      pub fn dispatch_resume(object: dispatch_object_t);
      pub fn dispatch_suspend(object: dispatch_object_t);

      
      pub fn dispatch_set_context(
        object: dispatch_object_t,
        context: *mut c_void
      );

      pub fn dispatch_release(object: dispatch_object_t);
      pub fn dispatch_retain(object: dispatch_object_t);

      pub static _dispatch_source_type_data_add: dispatch_source_type_t;

      pub fn dispatch_source_create(
        source_type: dispatch_source_type_t,
        handle: *const c_void,
        mask: u64,
        queue: dispatch_queue_t
      ) -> dispatch_source_t;

      pub fn dispatch_source_set_event_handler_f(
        source: dispatch_source_t,
        handler: extern "C" fn(*mut c_void)
      );

      pub fn dispatch_source_merge_data(
        source: dispatch_source_t,
        data: u64
      );

      pub fn dispatch_source_cancel(source: dispatch_source_t);
    }
  }

  pub struct WakeupBuilder {
    queue: ffi::dispatch_queue_t
  }

  impl WakeupBuilder {
    pub fn new() -> WakeupBuilder {
      WakeupBuilder {
        queue: unsafe {ffi::dispatch_get_main_queue()}
      }
    }
  }

  impl super::WakeupBuilder for WakeupBuilder {
    fn create_wakeup_channel<'a>(&mut self, callback: &'a mut FnMut()) -> (Box<super::WakeupSender>, Box<super::WakeupReceiver<'a>>) {
      let evt_source = unsafe {
        ffi::dispatch_source_create(
          ffi::_dispatch_source_type_data_add,
          0 as *const c_void, 0, self.queue)
      };
      //both the sender and receiver will release the event source,
      //so we need to have it retained twice
      //dispatch_source_create already retains it once,
      //so we need 1 other retain 
      unsafe {
        ffi::dispatch_retain(evt_source)
      };

      let sender : Box<super::WakeupSender> = Box::new(WakeupSender {
        evt_source: evt_source
      });
      let impl_trait_obj : Box<Drop> = Box::new(
        WakeupReceiverImpl { evt_source: evt_source }
      );
      let receiver = Box::new(super::WakeupReceiver {
        impl_ptr: impl_trait_obj,
        callback: callback
      });

      let receiver_ptr = Box::into_raw(receiver);
      //alias the pointer, which is safe because we ensure
      //the context is not used after the box is freed
      //by setting the context to null in the receiver drop method
      let callback_context = receiver_ptr as *mut c_void;

      let receiver = unsafe { Box::from_raw(receiver_ptr) };

      unsafe {
        ffi::dispatch_set_context(evt_source, callback_context);
        ffi::dispatch_source_set_event_handler_f(evt_source, wakeup_event_handler);
        ffi::dispatch_resume(evt_source);
      }
      
      (sender, receiver)
    }
  }

  extern "C" fn wakeup_event_handler(context: *mut c_void) {
    //if the context has been set to null, it means the receiver was dropped
    //this can only happen in the small peroid of time between
    //requesting the event source to be canceled by dispatch_source_cancel
    //and the time when it actually is canceled and guaranteed to not callback
    //the event handler anymore. The right thing to do is to drop the event since
    //the receiving part has been dropped.
    if context == std::ptr::null_mut() as *mut c_void {
      return;
    }

    let receiver : &mut super::WakeupReceiver = unsafe {
      &mut *(context as *mut super::WakeupReceiver)
    };
    receiver.invoke();
  }

  struct WakeupReceiverImpl {
    evt_source: ffi::dispatch_source_t
  }

  impl Drop for WakeupReceiverImpl {
    fn drop(&mut self) {
      unsafe {
        //cancel is async, so set context to null to
        //detect receiver has been dropped in callback
        //if it gets called before cancel takes effect
        ffi::dispatch_set_context(self.evt_source, std::ptr::null_mut() as *mut c_void);
        //cancel the source, which at some point
        //in the future will ensure the handler
        //won't be called anymore
        ffi::dispatch_source_cancel(self.evt_source);
        ffi::dispatch_release(self.evt_source);
      }
    }
  }

  struct WakeupSender {
    evt_source: ffi::dispatch_source_t,
  }

  impl super::WakeupSender for WakeupSender {
    fn wakeup(&self) {
      unsafe {
        ffi::dispatch_source_merge_data(self.evt_source, 1u64);
      }
    }
  }

  impl Drop for WakeupSender {
    fn drop(&mut self) {
      unsafe {
        ffi::dispatch_release(self.evt_source);
      }
    }
  }

}