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
git config --global alias.lesson_code "log --color --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s' --abbrev-commit --reverse"
```

## Struttura

Il corso e' strutturato in lezioni a cui sono associati differenti git branch
nominati secondo la seguente dicitura:  
`lesson-`**n** dove **n** e' il numero della lezione conrrispondente

per ottenere la lista delle lezioni con il relativo hash e il titolo della lezione
diigitare dalla propria shell (su windows eseguire da cmd.exe):

```bash
git branch -l
```
esempio di output:
```
* lesson-1
* lesson-2 
```

per poter navigare il codice relativo alla lezione a cui si e' interessati, digitare:

`git checkout lesson-`**n** dove **n** e' il numero della lezione a cui si e' interessati.

esempio per poter andare alla lezione 1:
```bash
git checkout lesson-1
```

per avere la lista del codice una volta effettuato il checkout nel branch relativo alla lezione, digitare:
```bash
git lesson_code
```
esempio di output:
```
1e2f9df - Introduzione al linguaggio di programmazione Rust.
9a3f2a5 - (HEAD -> lesson-1, origin/lesson-1) Variable declaration examples
```

Per accedere al codice della sezione a cui si e' interessati, digitare:
`git checkout `**hash** dove **hash** corrisponde alla prima colonna dell' output sopra indicato.


esempio, per andare alla sezione dedicata agli esempi delle dichiarazioni delle variabili (Variable declaration examples), digitare:
```bash
git checkout 9a3f2a5
```