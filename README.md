# AlvariumAuthorConsole
A PoC console implementation for spawning an Iota Streams Author instance. By running the service, 
a new Streams channel will be initiated and an api port opened for commands. 

### Instructions
#### Configuration
Adjust the `config.json` file to match the configuration of your node structure.

Example: 
```
{
  "node": "http://localhost:14265",
  "mwm": 5,
  "local_pow": true,
  "api_port": 8080,
  "seed": null
}
```

#### Running 
Firstly, start a new Author instance and listener services with: 
`cargo run`

This will return something like the following: 
```
Making Streams channel...

Channel Address - 2cd768499b14cbdb4f9d5c0fcd2bd0f0089d7729e2bb12c2e48bbb877a17672c0000000000000000:82bff4907f5cfdb84786de26
                        ^--- This is the Channel Root => AppInst:MsgId
Retrieval thread spawning. Searching for new messages...
API listening on http://0.0.0.0:8080
```

Next place the channel address into the AnnAddress field of the configurations on the simulator, then build and run it. 
The simulator will generate new subscribers and automatically communicate the subscription link to the Author Console. 
Once the author console sees this request, it will process it, and return a keyload link. The Subscriber should be able 
to see this new keyload message appear if they use a `get_next_msgs()` command (conducted authomatically in the 
`await_keyload()` function of the simulator's Subscriber class. Sensor readings and annotations can then
be sent as signed_packets over the tangle within the channel. The author will detect these new messages and store the 
annotations and readings accordingly. 


### Data Types 
See the models module to see the formatting of an `Annotation` and `Reading` as well as `Alvarium` based 
data types for compatibility with JSON structure of sdk. 

### Demo API 
Basic examples of available HTTP based curl commands

#### *get_channel_address*
Fetches the current channel application instance. 

##### Args
`N/A`
##### Command
`curl --location --request GET '127.0.0.1:8080/get_channel_address' --header 'Content-Type: application/json'`
##### Return 
Iota Streams `ChannelAddress/ApplicationInstance` for the current channel. 
```Channel Address: 2cd768499b14cbdb4f9d5c0fcd2bd0f0089d7729e2bb12c2e48bbb877a17672c0000000000000000```


##### *get_announcement_id*
Fetches the current channel announcement message address. 

##### Args
`N/A`
##### Command
`curl --location --request GET '127.0.0.1:8080/get_announcement_id' --header 'Content-Type: application/json'`
##### Return
Iota Streams `TangleAddress` for the Channel Announcement message.
```Announcement Id: 2cd768499b14cbdb4f9d5c0fcd2bd0f0089d7729e2bb12c2e48bbb877a17672c0000000000000000:82bff4907f5cfdb84786de26```


##### *subscribe*
Inform the Author instance of a new subscribing party, providing a message link for it to look for in the tangle. 
Once the message is retrieved and processed by the author, a keyload message will be generated for the new subscribed
party to begin messaging in.

##### Args
```
msgid: Streams MsgId of subscription message for author to process
pk: Hex string representation of subscribers ed25519 public key 
``` 
##### Command
`curl --location --request POST '127.0.0.1:8080/subscribe' --header 'Content-Type: application/json' 
--data-raw '{ "msgid": "30429f489e59579bd49768a3", "pk": "3d4d8b668e4a399e1ed8dd0bc4e0692cc80ca6d892c2cd7563a266e2ef24e4a8" }'
`
##### Return
Iota Streams sequence `TangleAddress` of the generated `Keyload` message. 
```Subscription processed, keyload link: <2cd768499b14cbdb4f9d5c0fcd2bd0f0089d7729e2bb12c2e48bbb877a17672c0000000000000000:3787799e7745c4603c344b70>```


##### *get_readings* 
Retrieve a list of all `Readings` associated with a given `SensorId`

##### Args
```
sensor_id: Hex string representation of subscribed sensor's ed25519 public key 
``` 
##### Command
`curl --location --request POST '127.0.0.1:8080/subscribe' --header 'Content-Type: application/json' 
--data-raw '{ "sensor_id": "3d4d8b668e4a399e1ed8dd0bc4e0692cc80ca6d892c2cd7563a266e2ef24e4a8" }'
`
##### Return
Vector of `Readings` associated with the given `SensorId`. 
```
[
  {
    "sensor_id":"ae05dfbf86ff76361f9c1b0c02fa3143560766bc069cba16fc2ca5379664c1de",
    "reading_id":"01",
    "data":"Some Masked Data Here"
  },
  {
    "sensor_id":"ae05dfbf86ff76361f9c1b0c02fa3143560766bc069cba16fc2ca5379664c1de",
    "reading_id":"02",
    "data":"More Masked Data Here"
  }, 
  ...
]
```

##### *get_annotations*
Retrieve a list of all `Annotations` associated with a given `ReadingId`

##### Args
```
reading_id: identifier for a specific reading  
``` 
##### Command
`curl --location --request GET '127.0.0.1:8080/get_annotations' --header 'Content-Type: application/json' 
--data-raw '{ "reading_id": "02" }'`
##### Return
Vector of `Annotations` associated with the given `ReadingId`. 
```
[
  {
    "reading_id":"02",
    "annotation": {
      "header": {
        "alg":"RS256",
        "typ":"JWT"
      },
      "payload": {
        "iss":"HostName",
        "sub":"0123456789",
        "iat":"1602350950120",
        "jti":"0987654321",
        "ann":"pki",
        "avl":1
      },
      "signature": "ABC123"
    }
  },
  ...
]
```

##### *get_confidence_score*
Retrieve a list of all `Annotations` associated with a given `ReadingId`

##### Args
```
reading_id: identifier for a specific reading  
``` 
##### Command
`curl --location --request GET '127.0.0.1:8080/get_confidence_score' --header 'Content-Type: application/json' 
--data-raw '{ "reading_id": "02" }'`
##### Return
Current `Confidence Score` based off the annotation values of associated annotations for a given `ReadingId`. 
```
confidence_score: 8
```


##### *get_filtered_annotations*
Retrieve a list of all `Annotations` filtered by the provided arguments

##### Args
```
<Optional> iss: Host name of the annotation issuing machine 
<Optional> sub: The ID/key of the application data that is being annotated
<Optional> iat: Timestamp for annotation generation 
<Optional> jti: Unique Json Web Token ID for annotation 
<Optional> ann: Type of annotation  
``` 

##### Command
`curl --location --request GET '127.0.0.1:8080/get_filtered_annotations' --header 'Content-Type: application/json' 
--data-raw '{ "iss": null, "sub": "0123456789", "iat": null, "jti": null, "ann": null }'`
##### Return
Vector of filtered `Annotations`. 
```
[
  {
    "reading_id":"02",
    "annotation": {
      "header": {
        "alg":"RS256",
        "typ":"JWT"
      },
      "payload": {
        "iss":"HostName",
        "sub":"0123456789",
        "iat":"1602350950120",
        "jti":"0987654321",
        "ann":"pki",
        "avl":1
      },
      "signature": "ABC123"
    }
  },
  ...
]
```

