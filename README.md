# Celularni automati - Conway’s Game of Life

## Opis problema
Game of Life predstavlja klasičan primer celularnog automata, koji se sastoji od dvodimenzionalne mreže ćelija. Svaka ćelija u mreži može biti u jednom od dva stanja: živa ili mrtva. Evolucija sistema odvija se kroz iteracije (generacije), pri čemu se stanje svake ćelije u narednoj iteraciji određuje na osnovu stanja njenih neposrednih suseda.

Problem koji se razmatra u okviru ovog projekta jeste efikasna simulacija evolucije velikog broja ćelija kroz veliki broj iteracija, kao i analiza performansi sekvencijalnih i paralelnih implementacija ovog problem.

Zbog prirode zavisnosti između ćelija i uniformnosti operacija koje se izvršavaju nad svakom ćelijom, Game of Life predstavlja izuzetno pogodan problem za paralelno izvršavanje

## Pravila igre
Za svaku ćeliju u mreži, u svakoj iteraciji, primenjuju se sledeća pravila:

1. Ako je ćelija živa i ima manje od dva živa suseda, ona umire (nedovoljna populacija).
2. Ako je ćelija živa i ima dva ili tri živa suseda, ona ostaje živa.
3. Ako je ćelija živa i ima više od tri živa suseda, ona umire (prevelika populacija).
4. Ako je ćelija mrtva i ima tačno tri živa suseda, ona postaje živa.

Sva ažuriranja stanja ćelija izvršavaju se istovremeno, na osnovu stanja mreže iz prethodne iteracije.

## Rešenje i korišćene metode
U okviru projekta postojaće:

### Python implementacija
1. Sekvencijalna verzija simulacije.
2. Paralelizovana verzija korišćenjem biblioteke multiprocessing

### Rust implementacija
1. Sekvencijalna verzija simulacije.
2. Paralelizovana verzija zasnovana na upotrebi niti.

Rezultati simulacije se čuvaju u datotekama koje predstavljaju stanje sistema po iteracijama.

### Analiza performansi
Za obe implementacije (Python i Rust) primeniće se eksperimenti jakog i slabog skaliranja, sa ciljem merenja ubrzanja paralelnih verzija u odnosu na sekvencijalne.

### Vizualizacija
Na osnovu prethodno generisanih datoteka koje opisuju stanje sistema po iteracijama, implementiraće se vizualizacija Game of Life igre u Rust okruženju, korišćenjem odgovarajuće grafičke biblioteke.
