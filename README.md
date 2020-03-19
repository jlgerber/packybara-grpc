# packybara-grpc

Grpc api, client and server wrapping the packybara async database api. The goal is to mirror the functionality of packybara-cli via grpc. 

# Notes

## Adding a Find 
update proto
update lib with proto structs
uncommend PbFind::<name> in bin/client and potentially remove "all_"
update bin/mod
create bin/cmd/<name>
update client.rs adding 
    use crate::proto stuff
    use packybara::db::find_all::...
    get_platforms mod and impl
    
create service/get_<name>
update service.rs with use 
update service.rs with mod get_<nam3>
update service.rs with get_<anme> async func