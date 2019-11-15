# Corso Introduttivo Rust

[programma](https://github.com/RustRome/corso-rust/blob/master/Programma.md)

## Prerequisiti

Per poter usurfruire del codice fornito durante il corso e' neccessario
aver installato git, la documentazione su come installare il tool git per
le varie piattaforme e' disponibile al seguente indirizzo:
[installing git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

### Configurazione git per navigazione repository corso

eseguire da shell (su windows eseguire da cmd.exe):

```bash
git config --global alias.lesson-code "log --color --pretty=format:'%Cred%h%Creset - %s' --abbrev-commit --reverse"
```

## Struttura

Il corso e' strutturato in lezioni a cui sono associati differenti git branch
nominati secondo la seguente dicitura:  
`lesson-`**n** dove **n** e' il numero della lezione conrrispondente

per ottenere la lista delle lezioni digitare dalla propria shell (su windows eseguire da cmd.exe):

```bash
git branch -l
```
esempio di output:
```
* lesson-1
* lesson-2
* lesson-3
* lesson-4
...
```

per poter navigare il codice relativo alla lezione a cui si e' interessati, digitare:

`git checkout lesson-`**n** dove **n** e' il numero della lezione a cui si e' interessati.

esempio per poter andare alla lezione 1:
```bash
git checkout lesson-1
```

per avere la lista del codice una volta effettuato il checkout nel branch relativo alla lezione, digitare:
```bash
git lesson-code
```
esempio di output:
```
1e2f9df - Introduzione al linguaggio di programmazione Rust.
9a3f2a5 - Variable declaration examples
...
```

Per accedere al codice della sezione a cui si e' interessati, digitare:
`git checkout `**hash** dove **hash** corrisponde alla prima colonna di colore rosso dell' output ottenuto con il comando sopra indicato.


esempio, per andare alla sezione dedicata agli esempi delle dichiarazioni delle variabili (Variable declaration examples), digitare:
```bash
git checkout 9a3f2a5
```
