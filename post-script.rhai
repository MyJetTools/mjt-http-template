let is_use_psql = variable::get("is_use_psql");
let is_use_nosql = variable::get("is_use_nosql");
let is_use_grpc_client = variable::get("is_use_grpc_client");
let is_use_sb = variable::get("is_use_sb");

if !is_use_psql{
    file::delete("./src/db_repositories");
}

if is_use_nosql == "no"{
    file::delete("./src/nosql");
}

if !is_use_grpc_client{
    file::delete("./src/grpc_client");
    file::delete("./proto");
    file::delete("./build.rs");
}

if is_use_sb != "subscriber" && is_use_sb != "both"  {
    file::delete("./src/sb/test_event/test_event_listener.rs");
}

if is_use_sb == "no"{
    file::delete("./src/sb");
}