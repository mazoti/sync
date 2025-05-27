//! Portuguese string messages module for sync operations
//!
//! This module contains all user-facing messages used throughout the application,
//! organized by category for better maintainability.

pub mod errors {
    pub const CONFIG_DUPLICATED: &str = "origem e destino já estão no arquivo de configuração";
    pub const CONFIG_EXT_CODE: &str = "arquivo de configuração deve terminar com .config";
    pub const CONFIG_FOLDER_CODE: &str =
        "configuração deve ser um arquivo de texto terminado em .config";
    pub const COPY_FILE_FOLDER: &str = "não foi possível copiar o arquivo para a pasta de destino";
    pub const DEST_FILE: &str = "arquivo de destino existe";
    pub const DEST_NOT_FILE: &str = "origem é um arquivo e destino uma pasta";
    pub const DEST_NOT_FOLDER: &str = "origem é uma pasta e destino um arquivo";
    pub const DIFF_FILE_FOLDER: &str = "arquivos ou pastas são diferentes";
    pub const FILE_SIZE: &str = "tamanho do arquivo deve ser positivo";
    pub const IO: &str = "erro de entrada ou saída";
    pub const OSSTRING: &str = "erro de string do sistema operacional";
    pub const PARSE_INT: &str = "não foi possível converter número para inteiro";
    pub const PARSE_LINE: &str = "não foi possível processar registro do arquivo de configuração";
    pub const SAME_FILE_FOLDER: &str = "origem e destino são os mesmos";
    pub const SOURCE_FILE: &str = "arquivo de origem não encontrado";
    pub const SOURCE_FOLDER: &str = "pasta de origem não encontrada";
    pub const SPLIT_SIZE: &str = "não é necessário dividir o arquivo";
    pub const SYSTEM_TIME: &str = "erro no relógio do sistema";
    pub const THREAD_JOIN: &str = "não foi possível terminar a thread";
    pub const TRY_FROM_INT: &str = "não foi possível converter número para usize";
}
pub mod status {
    pub const CREATE: &str = "Criando";
    pub const COPY: &str = "Copiando";
    pub const DUPLICATE: &str = "DUPLICADO";
    pub const ELAPSED: &str = "total:";
    pub const EMPTY: &str = "(VAZIO)";
    pub const ERROR: &str = "ERRO";
    pub const LOADING: &str = "Carregando";
    pub const OK: &str = "Ok";
    pub const ONE_ITEM: &str = "(UM ITEM)";
    pub const REMOVE: &str = "Apagando";
    pub const SIMULATION: &str = "(SIMULAÇÃO)";
    pub const START: &str = "iniciado";
    pub const SYNC: &str = "Sincronizando";
    pub const UPDATE: &str = "Atualizando";
    pub const USAGE: &str = "Uso:";
}

/// Message displayed when user enters "help" or sync didn't find any possible operation
pub const HELP: &str = "
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
