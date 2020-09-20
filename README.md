# rs_wowo_url
link shortening tool

Just a redis rust_lang tryout. A tool to shorten links with CLI api.

Required a redis installation in order to run.

-s <uri> to Save uri
-o <hash> to open default browser with uri from redis
-d <hash> to delete given hash
-clear to clear whole redis DB
-list to list all key/value pairs

No error handling yet so expect panic! at the disco.
