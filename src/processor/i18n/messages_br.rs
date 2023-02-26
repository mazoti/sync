//! Portuguese string messages crate

/// "Copying"
pub const COPY_MSG: &str = "Copiando";

/// "Creating"
pub const CREATE_MSG: &str = "Criando";

/// "DUPLICATED"
pub const DUPLICATE_MSG: &str = "DUPLICADO";

/// "elapsed:"
pub const ELAPSE_MSG: &str = "total:";

/// "(EMPTY)"
pub const EMPTY_MSG: &str = "(VAZIO)";

/// "ERROR"
pub const ERROR_MSG: &str = "ERRO";

/// Message displayed when user enters "help" or sync didn't find any possible operation
pub const HELP_MSG: &str = "
	sync [origem] [destino]
	sync [origem] [destino] [arquivo.config]
	sync [arquivo.config] (ou somente sync se os arquivos .config estão na mesma pasta)
	sync check [origem] [destino]
	sync empty [pasta]
	sync duplicate [pasta]
	sync force [origem] [destino]
	sync hash [pasta] [arquivo.hashs]
	sync hash [arquivo.hashs]
	sync join [pasta]
	sync move [origem] [destino]
	sync split [tamanho em bytes] [arquivo]
	sync simulate [origem] [destino]
";

/// "Carregando"
pub const LOADING_MSG: &str = "Carregando";

/// "Ok"
pub const OK_MSG: &str = "Ok";

/// "(ONE ITEM)"
pub const ONE_ITEM_MSG: &str = "(UM ITEM)";

/// "Removing"
pub const REMOVE_MSG: &str = "Apagando";

/// "(SIMULATION)"
pub const SIMULATION_MSG: &str = "(SIMULAÇÃO)";

/// "started"
pub const START_MSG: &str = "iniciado";

/// "Sync"
pub const SYNC_MSG: &str = "Sincronizando";

/// "Updating"
pub const UPDATE_MSG: &str = "Atualizando";

/// "Usage:"
pub const USAGE_MSG: &str = "Uso:";

/// "source and destination already in config file"
pub const ERROR_CONFIG_DUPLICATED: &str = "origem e destino já estão no arquivo de configuração";

/// "config file not ended in .config"
pub const ERROR_CONFIG_EXT_CODE: &str = "arquivo de configuração deve terminar com .config";

/// "config must be a .config text file"
pub const ERROR_CONFIG_FOLDER_CODE: &str =
    "configuração deve ser um arquivo de texto terminado em .config";

/// "cannot copy file to destination folder"
pub const ERROR_COPY_FILE_FOLDER: &str =
    "não foi possível copiar o arquivo para a pasta de destinor";

/// "destination file exists"
pub const ERROR_DEST_FILE: &str = "arquivo de destino existe";

/// "source is a file and destination is a folder"
pub const ERROR_DEST_NOT_FILE: &str = "origem é um arquivo e destino uma pasta";

/// "source is a folder and destination is a file"
pub const ERROR_DEST_NOT_FOLDER: &str = "origem é uma pasta e destino um arquivo";

/// "files or folders are different"
pub const ERROR_DIFF_FILE_FOLDER: &str = "arquivos ou pastas são diferentes";

/// "file size must be positive"
pub const ERROR_FILE_SIZE: &str = "tamanho do arquivo deve ser positivo";

/// "Input or output error"
pub const ERROR_IO: &str = "erro de entrada ou saída";

/// "Operating system string error"
pub const ERROR_OSSTRING: &str = "erro de string do sistema operacional";

/// "cannot convert number to integer"
pub const ERROR_PARSE_INT: &str = "não foi possível converter número para inteiro";

/// "cannot parse line from config file"
pub const ERROR_PARSE_LINE: &str = "não foi possível processar registro do arquivo de configuração";

/// "source and destination are the same"
pub const ERROR_SAME_FILE_FOLDER: &str = "origem e destino são os mesmos";

/// "source file not found"
pub const ERROR_SOURCE_FILE: &str = "arquivo de origem não encontrado";

/// "source folder not found"
pub const ERROR_SOURCE_FOLDER: &str = "pasta de origem não encontrada";

/// "file does not need to split"
pub const ERROR_SPLIT_SIZE: &str = "não é necessário dividir o arquivo";

/// "system time error"
pub const ERROR_SYSTEM_TIME: &str = "erro na hora do sistema";

/// "cannot join thread"
pub const ERROR_THREAD_JOIN: &str = "não foi possível terminar a thread";

/// "cannot convert number to usize"
pub const ERROR_TRY_FROM_INT: &str = "não foi possível converter número para usize";
