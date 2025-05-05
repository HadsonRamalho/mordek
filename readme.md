# mordek

mordek é uma aplicação em Rust que te permite realizar operações em arquivos e diretórios, como remoção, leitura e criação. Além disso, a aplicação reproduz áudios temáticos para cada operação, trazendo o Revenã de Ferro para dentro do seu terminal.

## Funcionalidades

- **Obliterar**: Remove arquivos ou diretórios do sistema.
- **Agarrar**: Lê o conteúdo de arquivos ou diretórios e procura por um texto específico.
- **Forjar**: Cria novos arquivos ou diretórios com o conteúdo especificado.
- **Setup**: Define o idioma do áudio (ainda em fase inicial).
- **Ajuda**: Lista os comandos disponíveis e como utilizá-los.

## Como Usar

### Comandos Disponíveis

1. **Obliterar**: Remove o alvo (arquivo ou diretório) do sistema.
   ```bash
   mordek obliterar <caminho>

2. **Agarrar**: Lê o conteúdo do alvo (arquivo ou diretório) e procura por um texto.
    ```bash
    mordek agarrar <caminho> <texto>

3. **Forjar**: Cria um novo arquivo ou diretório com o conteúdo especificado.
    ```bash
    mordek forjar <caminho> <conteúdo>

4. **Setup**: Exibe as opções de idiomas disponíveis (ainda em fase inicial).
    ```bash
    mordek setup

5. **Ajuda**: Exibe os comandos disponíveis.
    ```bash
    mordek help

5. **Ajuda Expandida**: Exibe a sintaxe de uso de um comando.
    ```bash
    mordek forjar --help

## Exemplos de Uso

### Remover um arquivo:
    mordek obliterar arquivo.txt

### Procurar por um texto em um arquivo:
    mordek agarrar arquivo.txt "texto a procurar"

### Procurar por um texto em um diretório:
    mordek agarrar minha_pasta "texto a procurar"

### Criar um arquivo com conteúdo:
    mordek forjar novo_arquivo.txt "conteúdo do arquivo"

### Criar um diretório e um arquivo dentro dele:
    mordek forjar novo_diretorio arquivo_dentro.txt

### Criar um diretório, um arquivo dentro dele, e inserir conteúdo no arquivo:
    mordek forjar novo_diretorio arquivo_dentro.txt "conteúdo do arquivo"