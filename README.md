# packybara-grpc

Grpc api, client and server wrapping the packybara async database api. The goal is to mirror the functionality of packybara-cli via grpc. 

# Notes

## Adding a Find 
* update proto
* update lib with proto structs
* uncomment `PbFind::<name>` in bin/client and        potentially remove "all_"
* update bin/mod
* create `bin/cmd/<name>`
* update client.rs adding 
    * use `crate::proto` stuff
    * use `packybara::db::find_all::...`
    * `get_platforms mod and impl`

* create `service/get_<name>`
* update service.rs with use 
* update service.rs with `mod get_<name>`
* update service.rs with `get_<name> async func`

# adding an Add

* uncomment `PbAdd::<name`> in bin/client 
  * potentially remove "all_"
  * remove `let tx...` line
  * replace `add(tx, cmd)` with `add(client,cmd)`
* update proto
* update lib with proto structs
* * update bin/mod
* create `bin/cmd/<name>`
* update client.rs adding 
    * use `crate::proto` stuff
    * use `packybara::db::add::...`
    * `get_platforms mod and impl`

* create `service/get_<name>`
* update service.rs with use 
* update service.rs with `mod get_<name>`
* update service.rs with `get_<name> async func`