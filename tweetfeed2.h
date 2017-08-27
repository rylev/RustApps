#include <stdlib.h>

typedef struct {} TweetFeedContext;

TweetFeedContext* tweetfeed_context_new_for_gtk(GMainContext* ui_ctx);
TweetFeedContext* tweetfeed_context_new_for_libdispatch(dispatch_queue_t ui_queue);
void tweetfeed_context_destroy(TweetFeedContext* ctx);

typedef struct {
  ByteSlice consumer_key;
  ByteSlice consumer_secret;
  ByteSlice token;
  ByteSlice token_secret;
} TweetFeedConfig;

typedef struct {} TweetFeedStream;

typedef struct {
  const uint8_t *bytes;
  size_t length;
} ByteSlice;

typedef struct {
  ByteSlice user_name;
  ByteSlice body;
} Tweet;

TweetFeedStream* tweetfeed_stream_new(TweetFeedContext* ctx, const TweetFeedConfig* cfg);
void tweetfeed_stream_start(TweetFeedStream* stream, void(TweetCallback*)(Tweet* tweet));
void tweetfeed_tweet_free(Tweet* tweet);
