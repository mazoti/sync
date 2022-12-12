pub const MSG_HELP: &str = "
	sync [origem] [destino]
	sync /home/user/data /home/user/backup
	sync \"C:\\Meu projeto\\arquivo.dat\" \"D:\\Backup\\arquivo.dat\"

O destino será criado caso não exista. Para simular a operação sem
nenhuma modificação, adicione a flag \"--simulate\":

    sync --simulate \"source\" \"destination\"

Para sincronizar mais arquivos ou pastas, crie um arquivo de configuração:

	sync [origem] [destino] [arquivo.config]
	sync /home/user/pasta /home/user/backup/pasta backups.config
	sync /home/user/arquivo.ext /home/user/backup/arquivo.ext backups.config
	...

e passe o arquivo de configuração como argumento:

	sync backups.config

Também são suportados vários arquivos de configuração:

	sync user1/folder   user1/backup/folder   sync_bin_folder/user1.config
	sync user1/file.ext user1/backup/file.ext sync_bin_folder/user1.config
	...
	sync user2/folder   user2/backup/folder   sync_bin_folder/user2.config
	sync user2/file.ext user2/backup/file.ext sync_bin_folder/user2.config
	...

para isso coloque os arquivos de configuração .config na mesma pasta
do sync e execute sem argumentos:

	sync

Para verificar todas as pastas e arquivos:

	sync --check \"origem\" \"destino\"

E para tentar novamente em caso de erros (até conseguir ou o usuário
entrar com ctrl+C), use a flag \"--force\":

	sync --force \"origem\" \"destino\"
";

pub const ERROR_MSGS: &[&str] = &[
    "origem e destino já estão no arquivo de configuração",           // ERROR_CONFIG_DUPLICATED
    "arquivo de configuração deve terminar com .config",              // ERROR_CONFIG_EXT_CODE
    "configuração deve ser um arquivo de texto terminado em .config", // ERROR_CONFIG_FOLDER_CODE
    "não foi possível copiar o arquivo para a pasta de destinor",     // ERROR_COPY_FILE_FOLDER
    "origem é um arquivo e destino uma pasta",                        // ERROR_DEST_NOT_FILE
    "origem é uma pasta e destino um arquivo",                        // ERROR_DEST_NOT_FOLDER
    "arquivos ou pastas são diferentes",                              // ERROR_DIFF_FILE_FOLDER
	"tamanho do arquivo deve ser positivo",                           // ERROR_FILE_SIZE
    "erro de entrada ou saída",                                       // ERROR_IO
    "não foi possível processar registro do arquivo de configuração", // ERROR_PARSE_LINE
    "origem e destino são os mesmos",                                 // ERROR_SAME_FILE_FOLDER
    "arquivo de origem não encontrado",                               // ERROR_SOURCE_FILE
    "pasta de destino não encontrada",                                // ERROR_SOURCE_FOLDER
    "não é necessário dividir o arquivo",                             // ERROR_SPLIT_SIZE
    
    "não foi possível gerar a saída",                                 // ERROR_OUTPUT
    "já está no arquivo de configuração",                             // ERROR_SOURCE_DUP
	"não foi possível parar a thread",                                // ERROR_THREAD_JOIN
];

pub const COMMAND_MSGS: &[&str] = &[
    "      Criando",
    "     Copiando",
    "Total",
    "ERRO",
    "   Carregando",
    "           Ok",
    "     Apagando",
    "iniciado",
    "Sincronizando",
    "  Atualizando",
    "Uso:",
    "ATENÇÃO",
    "(SIMULAÇÃO)",
];
