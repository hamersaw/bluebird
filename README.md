#bluebird

##Overview
Twitter API for rust using REST API provided by twitter 
(https://dev.twitter.com/rest/public). Functionality is extremely simple 
to implement so please feel free to raise an issue with any added actions. 
I would be happy to implement them.

##Functionality
- Show home timeline using count, since_id, or max_id
- Lookup multiple users by screen name or user id
- Open tweet stream filtered by users or location
- Search users by query
- Show user based on user id
- Post tweet
- Show users timeline

##Examples
See examples folder.

Now available as crate!
```toml
[dependencies]
bluebird = "*"
```

##TODO
- continue filling out twitter REST API functionality
