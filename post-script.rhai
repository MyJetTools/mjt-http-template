let is_use_psql = variable::get("is_use_psql");
let is_use_nosql = variable::get("is_use_nosql");
let is_use_grpc_client = variable::get("is_use_grpc_client");

if !is_use_psql{
    file::delete("./src/db_repositories");
}

if is_use_nosql == "no"{
    file::delete("./src/nosql");
}

if !is_use_grpc_client{
    file::delete("./src/grpc_client");
    file::delete("./proto");
}