var searchIndex = JSON.parse('{\
"kalavar_core":{"doc":"","i":[[5,"main","kalavar_core","",null,[[]]],[0,"managers","","",null,null],[0,"config","kalavar_core::managers","",null,null],[3,"ConfigManager","kalavar_core::managers::config","A utility structure designed to parse the content of the…",null,null],[12,"config_path","","(Platform dependant) path to the configuration file",0,null],[12,"config","","The actual configuration of the software",0,null],[3,"Config","","A structure containing the root keys of the config.toml file",null,null],[12,"network","","Network configuration options",1,null],[12,"processing","","",1,null],[12,"language","","",1,null],[3,"NetConfig","","Network configuration option utility structure",null,null],[12,"bind_port","","The port which the server should attempt to bind itself to",2,null],[12,"bind_addr","","The IP address which the server should attempt to bind to",2,null],[12,"max_connections","","The maximum number of connections the server should accept…",2,null],[12,"connections_per_thread","","The maximum number of connections that the server should…",2,null],[3,"ProcessingConfig","","",null,null],[12,"max_threads","","",3,null],[3,"LanguageConfig","","",null,null],[0,"logging","kalavar_core::managers","",null,null],[3,"LoggingManager","kalavar_core::managers::logging","",null,null],[12,"levels","","",4,null],[12,"log_file","","",4,null],[12,"log_file_full","","",4,null],[0,"connections","kalavar_core::managers","",null,null],[0,"connection","kalavar_core::managers::connections","",null,null],[3,"Connection","kalavar_core::managers::connections::connection","",null,null],[12,"id","","",5,null],[12,"opened","","",5,null],[12,"remote","","",5,null],[12,"stream","","",5,null],[12,"logger","","",5,null],[12,"last_ping","","",5,null],[12,"receiver","","",5,null],[12,"transmitter","","",5,null],[0,"connection_manager","kalavar_core::managers::connections","",null,null],[3,"ConnectionManager","kalavar_core::managers::connections::connection_manager","",null,null],[12,"listener","","",6,null],[12,"port","","",6,null],[12,"addr","","",6,null],[12,"connections","","",6,null],[12,"dbm","","",6,null],[0,"storage","kalavar_core::managers","",null,null],[3,"StorageManager","kalavar_core::managers::storage","",null,null],[12,"databases","","",7,null],[12,"last_write","","",7,null],[12,"dir","","",7,null],[0,"encryption","kalavar_core::managers","",null,null],[3,"EncryptionManager","kalavar_core::managers::encryption","",null,null],[12,"key","","",8,null],[12,"stream","","",8,null],[11,"new","","",8,[[["tcpstream",3]],["encryptionmanager",3]]],[11,"init","","",8,[[]]],[11,"write","","",8,[[]]],[11,"read","","",8,[[["bufreader",3]]]],[0,"analytics","kalavar_core::managers","",null,null],[0,"authentication","","",null,null],[3,"AuthManager","kalavar_core::managers::authentication","",null,null],[12,"permissions","","",9,null],[12,"remote","","",9,null],[12,"is_admin","","",9,null],[12,"is_root","","",9,null],[12,"username","","",9,null],[12,"certificate","","",9,null],[12,"validated","","",9,null],[0,"permission","kalavar_core::managers","",null,null],[3,"PermissionManager","kalavar_core::managers::permission","",null,null],[12,"inner","","",10,null],[3,"DatabasePermissions","","",null,null],[12,"int","","",11,null],[12,"create","","",11,null],[12,"delete","","",11,null],[12,"read","","",11,null],[12,"write","","",11,null],[12,"change_name","","",11,null],[12,"add_tables","","",11,null],[12,"remove_tables","","",11,null],[12,"modify_tables","","",11,null],[12,"access","","",11,null],[3,"TablePermissions","","",null,null],[12,"int","","",12,null],[12,"create","","",12,null],[12,"delete","","",12,null],[12,"read","","",12,null],[12,"write","","",12,null],[12,"change_name","","",12,null],[12,"add_columns","","",12,null],[12,"remove_columns","","",12,null],[12,"modify_columns","","",12,null],[0,"implementors","kalavar_core","",null,null],[0,"managers","kalavar_core::implementors","",null,null],[0,"config","kalavar_core::implementors::managers","",null,null],[5,"parse_config","kalavar_core::implementors::managers::config","",null,[[["file",3],["loggingmanager",3],["configmanager",3]]]],[17,"BASE_CONFIG","","",null,null],[17,"LINE_ENDING","","",null,null],[11,"new","kalavar_core::managers::config","",0,[[["loggingmanager",3]]]],[0,"logging","kalavar_core::implementors::managers","",null,null],[5,"format_date","kalavar_core::implementors::managers::logging","",null,[[]]],[17,"RESET","","",null,null],[17,"FG_RED","","",null,null],[17,"FG_YEL","","",null,null],[17,"FG_GRE","","",null,null],[17,"FG_CYA","","",null,null],[17,"FG_MAG","","",null,null],[17,"FG_GRY","","",null,null],[11,"new","kalavar_core::managers::logging","",4,[[],["loggingmanager",3]]],[11,"init","","",4,[[]]],[11,"fatal","","",4,[[["errormap",4],["display",8]]]],[11,"debug_message","","",4,[[["string",3],["into",8]]]],[11,"debug","","",4,[[["debug",8]]]],[11,"debug_pretty","","",4,[[["debug",8]]]],[11,"info","","",4,[[["display",8]]]],[11,"log","","",4,[[["display",8]]]],[11,"warn","","",4,[[["display",8]]]],[11,"error","","",4,[[["errormap",4],["display",8]]]],[0,"connections","kalavar_core::implementors::managers","",null,null],[0,"connection","kalavar_core::implementors::managers::connections","",null,null],[11,"new","kalavar_core::managers::connections::connection","",5,[[["loggingmanager",3],["connectionprotocolmessage",3],["receiver",3],["sender",3]]]],[0,"connection_manager","kalavar_core::implementors::managers::connections","",null,null],[11,"new","kalavar_core::managers::connections::connection_manager","",6,[[["configmanager",3],["loggingmanager",3]]]],[11,"connect","","",6,[[["loggingmanager",3]]]],[0,"storage","kalavar_core::implementors::managers","",null,null],[5,"parse_incoming","kalavar_core::implementors::managers::storage","",null,[[["storagemanager",3],["loggingmanager",3]]]],[5,"handle_missing_data_dir","","",null,[[["errorkind",4],["string",3],["loggingmanager",3]]]],[11,"new","kalavar_core::managers::storage","",7,[[["loggingmanager",3]]]],[0,"core_structures","kalavar_core","",null,null],[0,"database_record","kalavar_core::core_structures","",null,null],[3,"DatabaseRecord","kalavar_core::core_structures::database_record","",null,null],[12,"name","","",13,null],[12,"tables","","",13,null],[12,"backing","","",13,null],[12,"backing_is_dir","","",13,null],[11,"new","","",13,[[["string",3]],["databaserecord",3]]],[11,"new_table","","",13,[[["vec",3],["string",3]]]],[11,"read_table","","",13,[[]]],[0,"table","kalavar_core::core_structures","",null,null],[3,"Table","kalavar_core::core_structures::table","",null,null],[12,"name","","",14,null],[12,"columns","","",14,null],[12,"rows","","",14,null],[11,"new","","",14,[[["string",3],["vec",3],["into",8]],["table",3]]],[0,"query","kalavar_core::core_structures","",null,null],[0,"query_result","","",null,null],[0,"column","","",null,null],[3,"ColumnType","kalavar_core::core_structures::column","",null,null],[12,"is_private","","",15,null],[12,"inner_type","","",15,null],[12,"min_len","","",15,null],[12,"max_len","","",15,null],[12,"value_type","","",15,null],[4,"ColumnTypeEnum","","",null,null],[13,"String","","",16,null],[13,"JSON","","",16,null],[13,"Bool","","",16,null],[13,"Byte","","",16,null],[13,"BLOB","","",16,null],[13,"Integer8","","",16,null],[13,"Integer16","","",16,null],[13,"Integer32","","",16,null],[13,"Integer64","","",16,null],[13,"SignedInteger8","","",16,null],[13,"SignedInteger16","","",16,null],[13,"SignedInteger32","","",16,null],[13,"SignedInteger64","","",16,null],[13,"BigInteger","","",16,null],[13,"SignedBigInteger","","",16,null],[13,"Snowflake","","",16,null],[13,"UUID","","",16,null],[13,"SonyFlake","","",16,null],[13,"RGB","","",16,null],[13,"RGBA","","",16,null],[13,"CMYK","","",16,null],[13,"Hex","","",16,null],[13,"Array","","",16,null],[13,"Enum","","",16,null],[13,"IPv4","","",16,null],[13,"IPv6","","",16,null],[13,"Mac","","",16,null],[13,"Mac8","","",16,null],[13,"Timestamp","","",16,null],[13,"NaiveTimestamp","","",16,null],[5,"process_column","","",null,[[["columntypeenum",4],["columntype",3]],["columntype",3]]],[17,"BIT","","",null,null],[17,"BYTE","","",null,null],[17,"KB","","",null,null],[17,"MB","","",null,null],[17,"GB","","",null,null],[11,"new","","",15,[[["columntypeenum",4],["option",4]],["columntype",3]]],[11,"new_prv","","",15,[[["columntypeenum",4],["option",4]],["columntype",3]]],[0,"row","kalavar_core::core_structures","",null,null],[3,"Row","kalavar_core::core_structures::row","",null,null],[12,"entries","","",17,null],[3,"Cell","","",null,null],[12,"inner_type","","",18,null],[12,"inner_value","","",18,null],[11,"new","","",17,[[["vec",3]],["row",3]]],[11,"populate","","",17,[[["asbytes",8],["string",3]]]],[0,"connection_protocol","kalavar_core::core_structures","",null,null],[3,"ConnectionProtocolMessage","kalavar_core::core_structures::connection_protocol","",null,null],[12,"recipient","","",19,null],[12,"sender","","",19,null],[12,"inner_type","","",19,null],[12,"inner","","",19,null],[12,"opcode","","",19,null],[4,"DataType","","",null,null],[13,"String","","",20,null],[13,"Bytes","","",20,null],[13,"MEM","","",20,null],[13,"Struct","","",20,null],[11,"new_mem","","",19,[[],["connectionprotocolmessage",3]]],[11,"new_con","","",19,[[],["connectionprotocolmessage",3]]],[0,"as_bytes","kalavar_core::core_structures","",null,null],[8,"AsBytes","kalavar_core::core_structures::as_bytes","",null,null],[10,"as_kv_bytes","","",21,[[],["vec",3]]],[0,"table_record","kalavar_core::core_structures","",null,null],[3,"TableRecord","kalavar_core::core_structures::table_record","",null,null],[12,"inner","","",22,null],[12,"start","","",22,null],[12,"end","","",22,null],[12,"backing","","",22,null],[12,"backing_is_dir","","",22,null],[12,"name","","",22,null],[12,"columns","","",22,null],[12,"rows","","",22,null],[11,"new","","",22,[[["vec",3]],["tablerecord",3]]],[0,"row_record","kalavar_core::core_structures","",null,null],[3,"RowRecord","kalavar_core::core_structures::row_record","",null,null],[12,"inner","","",23,null],[12,"start","","",23,null],[12,"end","","",23,null],[12,"id","","",23,null],[12,"backing","","",23,null],[12,"backing_is_dir","","",23,null],[12,"columns","","",23,null],[0,"core","kalavar_core","",null,null],[0,"utils","kalavar_core::core","",null,null],[0,"connection_handling","kalavar_core::core::utils","",null,null],[5,"handle","kalavar_core::core::utils::connection_handling","The following content is undocumented due to not being…",null,[[["connection",3]]]],[0,"api","","",null,null],[0,"opcode_parser","kalavar_core::core::utils::connection_handling::api","",null,null],[4,"OpCodes","kalavar_core::core::utils::connection_handling::api::opcode_parser","",null,null],[13,"Normal","","",24,null],[13,"Ping","","",24,null],[13,"Pong","","",24,null],[13,"Disconnect","","",24,null],[13,"Reconnect","","",24,null],[13,"Shutdown","","",24,null],[13,"Status","","",24,null],[13,"Hello","","",24,null],[13,"Authenticate","","",24,null],[13,"Ready","","",24,null],[13,"Connect","","",24,null],[13,"MemUse","","",24,null],[13,"Terminate","","",24,null],[13,"PayloadTamper","","",24,null],[0,"errors","kalavar_core","",null,null],[3,"GeneralError","kalavar_core::errors","",null,null],[12,"code","","",25,null],[4,"ErrorMap","","",null,null],[13,"GXXX","","",26,null],[13,"G101","","",26,null],[13,"G201","","",26,null],[13,"G202","","",26,null],[13,"G203","","",26,null],[13,"E204","","",26,null],[7,"ALLOC","kalavar_core","",null,null],[11,"from","kalavar_core::managers::config","",0,[[]]],[11,"into","","",0,[[]]],[11,"to_owned","","",0,[[]]],[11,"clone_into","","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"into","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","","",2,[[]]],[11,"into","","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"into","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","","",27,[[]]],[11,"into","","",27,[[]]],[11,"borrow","","",27,[[]]],[11,"borrow_mut","","",27,[[]]],[11,"try_from","","",27,[[],["result",4]]],[11,"try_into","","",27,[[],["result",4]]],[11,"type_id","","",27,[[],["typeid",3]]],[11,"from","kalavar_core::managers::logging","",4,[[]]],[11,"into","","",4,[[]]],[11,"to_owned","","",4,[[]]],[11,"clone_into","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","kalavar_core::managers::connections::connection","",5,[[]]],[11,"into","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","kalavar_core::managers::connections::connection_manager","",6,[[]]],[11,"into","","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","kalavar_core::managers::storage","",7,[[]]],[11,"into","","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","kalavar_core::managers::encryption","",8,[[]]],[11,"into","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","kalavar_core::managers::authentication","",9,[[]]],[11,"into","","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"from","kalavar_core::managers::permission","",10,[[]]],[11,"into","","",10,[[]]],[11,"to_string","","",10,[[],["string",3]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","","",11,[[]]],[11,"into","","",11,[[]]],[11,"borrow","","",11,[[]]],[11,"borrow_mut","","",11,[[]]],[11,"try_from","","",11,[[],["result",4]]],[11,"try_into","","",11,[[],["result",4]]],[11,"type_id","","",11,[[],["typeid",3]]],[11,"from","","",12,[[]]],[11,"into","","",12,[[]]],[11,"borrow","","",12,[[]]],[11,"borrow_mut","","",12,[[]]],[11,"try_from","","",12,[[],["result",4]]],[11,"try_into","","",12,[[],["result",4]]],[11,"type_id","","",12,[[],["typeid",3]]],[11,"from","kalavar_core::core_structures::database_record","",13,[[]]],[11,"into","","",13,[[]]],[11,"borrow","","",13,[[]]],[11,"borrow_mut","","",13,[[]]],[11,"try_from","","",13,[[],["result",4]]],[11,"try_into","","",13,[[],["result",4]]],[11,"type_id","","",13,[[],["typeid",3]]],[11,"from","kalavar_core::core_structures::table","",14,[[]]],[11,"into","","",14,[[]]],[11,"to_owned","","",14,[[]]],[11,"clone_into","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"from","kalavar_core::core_structures::column","",15,[[]]],[11,"into","","",15,[[]]],[11,"to_owned","","",15,[[]]],[11,"clone_into","","",15,[[]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"from","","",16,[[]]],[11,"into","","",16,[[]]],[11,"to_owned","","",16,[[]]],[11,"clone_into","","",16,[[]]],[11,"borrow","","",16,[[]]],[11,"borrow_mut","","",16,[[]]],[11,"try_from","","",16,[[],["result",4]]],[11,"try_into","","",16,[[],["result",4]]],[11,"type_id","","",16,[[],["typeid",3]]],[11,"from","kalavar_core::core_structures::row","",17,[[]]],[11,"into","","",17,[[]]],[11,"to_owned","","",17,[[]]],[11,"clone_into","","",17,[[]]],[11,"borrow","","",17,[[]]],[11,"borrow_mut","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"try_into","","",17,[[],["result",4]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"from","","",18,[[]]],[11,"into","","",18,[[]]],[11,"to_owned","","",18,[[]]],[11,"clone_into","","",18,[[]]],[11,"borrow","","",18,[[]]],[11,"borrow_mut","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"try_into","","",18,[[],["result",4]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"from","kalavar_core::core_structures::connection_protocol","",19,[[]]],[11,"into","","",19,[[]]],[11,"to_owned","","",19,[[]]],[11,"clone_into","","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"try_into","","",19,[[],["result",4]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"from","","",20,[[]]],[11,"into","","",20,[[]]],[11,"to_owned","","",20,[[]]],[11,"clone_into","","",20,[[]]],[11,"borrow","","",20,[[]]],[11,"borrow_mut","","",20,[[]]],[11,"try_from","","",20,[[],["result",4]]],[11,"try_into","","",20,[[],["result",4]]],[11,"type_id","","",20,[[],["typeid",3]]],[11,"from","kalavar_core::core_structures::table_record","",22,[[]]],[11,"into","","",22,[[]]],[11,"borrow","","",22,[[]]],[11,"borrow_mut","","",22,[[]]],[11,"try_from","","",22,[[],["result",4]]],[11,"try_into","","",22,[[],["result",4]]],[11,"type_id","","",22,[[],["typeid",3]]],[11,"from","kalavar_core::core_structures::row_record","",23,[[]]],[11,"into","","",23,[[]]],[11,"borrow","","",23,[[]]],[11,"borrow_mut","","",23,[[]]],[11,"try_from","","",23,[[],["result",4]]],[11,"try_into","","",23,[[],["result",4]]],[11,"type_id","","",23,[[],["typeid",3]]],[11,"from","kalavar_core::core::utils::connection_handling::api::opcode_parser","",24,[[]]],[11,"into","","",24,[[]]],[11,"to_owned","","",24,[[]]],[11,"clone_into","","",24,[[]]],[11,"borrow","","",24,[[]]],[11,"borrow_mut","","",24,[[]]],[11,"try_from","","",24,[[],["result",4]]],[11,"try_into","","",24,[[],["result",4]]],[11,"type_id","","",24,[[],["typeid",3]]],[11,"from","kalavar_core::errors","",25,[[]]],[11,"into","","",25,[[]]],[11,"to_string","","",25,[[],["string",3]]],[11,"borrow","","",25,[[]]],[11,"borrow_mut","","",25,[[]]],[11,"try_from","","",25,[[],["result",4]]],[11,"try_into","","",25,[[],["result",4]]],[11,"type_id","","",25,[[],["typeid",3]]],[11,"from","","",26,[[]]],[11,"into","","",26,[[]]],[11,"to_owned","","",26,[[]]],[11,"clone_into","","",26,[[]]],[11,"to_string","","",26,[[],["string",3]]],[11,"borrow","","",26,[[]]],[11,"borrow_mut","","",26,[[]]],[11,"try_from","","",26,[[],["result",4]]],[11,"try_into","","",26,[[],["result",4]]],[11,"type_id","","",26,[[],["typeid",3]]],[11,"clone","kalavar_core::managers::config","",0,[[],["configmanager",3]]],[11,"clone","kalavar_core::managers::logging","",4,[[],["loggingmanager",3]]],[11,"clone","kalavar_core::core_structures::table","",14,[[],["table",3]]],[11,"clone","kalavar_core::core_structures::column","",16,[[],["columntypeenum",4]]],[11,"clone","","",15,[[],["columntype",3]]],[11,"clone","kalavar_core::core_structures::row","",17,[[],["row",3]]],[11,"clone","","",18,[[],["cell",3]]],[11,"clone","kalavar_core::core_structures::connection_protocol","",20,[[],["datatype",4]]],[11,"clone","","",19,[[],["connectionprotocolmessage",3]]],[11,"clone","kalavar_core::core::utils::connection_handling::api::opcode_parser","",24,[[],["opcodes",4]]],[11,"clone","kalavar_core::errors","",26,[[],["errormap",4]]],[11,"fmt","kalavar_core::managers::config","",0,[[["formatter",3]],["result",6]]],[11,"fmt","kalavar_core::managers::logging","",4,[[["formatter",3]],["result",6]]],[11,"fmt","kalavar_core::managers::storage","",7,[[["formatter",3]],["result",6]]],[11,"fmt","kalavar_core::core_structures::database_record","",13,[[["formatter",3]],["result",6]]],[11,"fmt","kalavar_core::core_structures::table","",14,[[["formatter",3]],["result",6]]],[11,"fmt","kalavar_core::core_structures::column","",16,[[["formatter",3]],["result",6]]],[11,"fmt","","",15,[[["formatter",3]],["result",6]]],[11,"fmt","kalavar_core::core_structures::row","",17,[[["formatter",3]],["result",6]]],[11,"fmt","","",18,[[["formatter",3]],["result",6]]],[11,"fmt","kalavar_core::core_structures::connection_protocol","",20,[[["formatter",3]],["result",6]]],[11,"fmt","","",19,[[["formatter",3]],["result",6]]],[11,"fmt","kalavar_core::core_structures::table_record","",22,[[["formatter",3]],["result",6]]],[11,"fmt","kalavar_core::core_structures::row_record","",23,[[["formatter",3]],["result",6]]],[11,"fmt","kalavar_core::core::utils::connection_handling::api::opcode_parser","",24,[[["formatter",3]],["result",6]]],[11,"fmt","kalavar_core::errors","",26,[[["formatter",3]],["result",6]]],[11,"fmt","","",25,[[["formatter",3]],["result",6]]],[11,"fmt","kalavar_core::managers::permission","",10,[[["formatter",3]],["result",6]]],[11,"fmt","kalavar_core::errors","",26,[[["formatter",3]],["result",6]]],[11,"fmt","","",25,[[["formatter",3]],["result",6]]]],"p":[[3,"ConfigManager"],[3,"Config"],[3,"NetConfig"],[3,"ProcessingConfig"],[3,"LoggingManager"],[3,"Connection"],[3,"ConnectionManager"],[3,"StorageManager"],[3,"EncryptionManager"],[3,"AuthManager"],[3,"PermissionManager"],[3,"DatabasePermissions"],[3,"TablePermissions"],[3,"DatabaseRecord"],[3,"Table"],[3,"ColumnType"],[4,"ColumnTypeEnum"],[3,"Row"],[3,"Cell"],[3,"ConnectionProtocolMessage"],[4,"DataType"],[8,"AsBytes"],[3,"TableRecord"],[3,"RowRecord"],[4,"OpCodes"],[3,"GeneralError"],[4,"ErrorMap"],[3,"LanguageConfig"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);