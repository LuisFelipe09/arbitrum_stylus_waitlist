# arbitrum_stylus_waitlist

Se guadran los parque en espera de validacion para ser integrados en el whitelist del protocolo.

## Run wait list

Crear una archivo en la raiz de proyecto con nombre ´private.txt´ y guarde la llave privada de la billetera. Arbirtrum stylus funciona sobre artibrum sepolia por lo cual podemos solicitar eth de la faucet a nuesra direecion de metaMask.

Luego compilamos el contracto 

-- cargo stylus check

Si compila correcto podemos proceder a desplegar.

-- cargo stylus deploy --private-key-path=private.txt


