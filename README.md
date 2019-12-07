# tinyraycaster

Implementando [tinyraycaster](https://github.com/ssloy/tinyraycaster/wiki) em rust.

[Estado atual](output.mp4)

## Rodando o código

Clone o repositório e entre no diretório

`git clone https://github.com/fuzzyqu/tinyraycaster.git && cd tinyraycaster`

Para executar

`cargo r --release`

O raycaster ira criar criar 360 imagens dentro da pasta output.

Gerando um vídeo com as imagens

`ffmpeg -i "output/framebuffer_%d.png" -pix_fmt yuv420p output.mp4`
