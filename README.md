# Taller de Solana
En este taller cubrimos los primeros pasos para desarrollar un contrato inteligente (programa) en la red de Solana mediante el uso del framework [Anchor](https://www.anchor-lang.com).
El objetivo es adquirir los conocimientos básicos para poder inicial el camino y profundizar a partir de la documentación oficial.

En el taller desarrollaremos un programa de scoring que permita un solo voto por pares wallet-entidad a votar. Este programa
puede servir para implementar un sistema de reputación para distintos elementos como por ejemplo: usuarios, productos, servicios, etc.

## Reto
Modificar el program para que permita asignar una nota de 0 a 10 en lugar de las 4 opciones presentadas.

Otras mejoras que se pueden implementar:
- Dar peso a los votos, es decir, que un voto de un usuario tenga más peso que otro.
- Añadir una allow-list de wallets que puedan votar.