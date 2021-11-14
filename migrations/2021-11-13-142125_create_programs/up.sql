CREATE TABLE programs (
    id INTEGER NOT NULL PRIMARY KEY,
    keyword VARCHAR(64) NOT NULL,
    name VARCHAR(64) NOT NULL,
    locale VARCHAR(5) NULL
);

INSERT INTO programs VALUES(0, 'navigateur', 'firefox', 'fr_FR');
INSERT INTO programs VALUES(1, 'navigateur', 'chromium', 'fr_FR');
INSERT INTO programs VALUES(2, 'navigateur', 'chrome', 'fr_FR');
INSERT INTO programs VALUES(3, 'navigateur', 'brave', 'fr_FR');

INSERT INTO programs VALUES(0, 'browser', 'firefox');
INSERT INTO programs VALUES(1, 'browser', 'chromium');
INSERT INTO programs VALUES(2, 'browser', 'chrome');
INSERT INTO programs VALUES(3, 'browser', 'brave');
