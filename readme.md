# Mordek

Mordek é uma aplicação em Rust que realiza operações em arquivos e diretórios, como remoção, leitura e criação. Além disso, a aplicação reproduz áudios temáticos para cada operação, tornando a experiência mais imersiva.

## Funcionalidades

- **Obliterar**: Remove arquivos ou diretórios do sistema.
- **Agarrar**: Lê o conteúdo de arquivos ou diretórios e procura por um texto específico.
- **Forjar**: Cria novos arquivos ou diretórios com o conteúdo especificado.
- **Ajuda**: Lista os comandos disponíveis e como utilizá-los.

## Como Usar

### Comandos Disponíveis

1. **Obliterar**: Remove o alvo (arquivo ou diretório) do sistema.
   ```bash
   mordek --obliterar <caminho>

2. **Agarrar**: Lê o conteúdo do alvo (arquivo ou diretório) e procura por um texto.
    ```bash
    mordek --agarrar <caminho> <texto>

3. **Forjar**: Cria um novo arquivo ou diretório com o conteúdo especificado.
    ```bash
    mordek --forjar <caminho> <conteúdo>

4. **Ajuda**: Exibe os comandos disponíveis.
    ```bash
    mordek --help

## Exemplos de Uso

### Remover um arquivo:
    ```bash
    mordek --obliterar arquivo.txt

### Procurar por um texto em um arquivo:
    ```bash
    mordek --agarrar arquivo.txt "texto a procurar"

### Procurar por um texto em um diretório:
    ```bash
    mordek --agarrar minha_pasta "texto a procurar"

### Criar um arquivo com conteúdo:
    ```bash
    mordek --forjar novo_arquivo.txt "conteúdo do arquivo"

### Criar um diretório e um arquivo dentro dele:
    mordek --forjar novo_diretorio arquivo_dentro.txt

### Criar um diretório, um arquivo dentro dele, e inserir conteúdo no arquivo:
    mordek --forjar novo_diretorio arquivo_dentro.txt "conteúdo do arquivo"