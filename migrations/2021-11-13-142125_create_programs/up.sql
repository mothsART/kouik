CREATE TABLE programs (
    id INTEGER NOT NULL PRIMARY KEY,
    keyword VARCHAR(64) NOT NULL,
    name VARCHAR(64) NOT NULL,
    locale VARCHAR(5) NOT NULL
);

INSERT INTO programs VALUES(0, 'navigateur', 'firefox', 'fr_FR');
INSERT INTO programs VALUES(1, 'navigateur', 'chromium', 'fr_FR');
INSERT INTO programs VALUES(2, 'navigateur', 'chrome', 'fr_FR');
INSERT INTO programs VALUES(3, 'navigateur', 'brave', 'fr_FR');
