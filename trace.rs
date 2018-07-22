Trace für Debugging zwecke.
ähnlich wie ein Transaktionelles System gibt es subtraces.
Ein Trace besteht aus einem Infobaustein der alle informationen zu dem Vorgang enthält
und eine Kette an Events.
Ein Event kann ein Event sein oder ein Verweis an eine SubKette.

Jede Kette kann einen von 3 Zuständen haben.
Pending
Saving,
Deleting.

Wenn eine Kette den Status Pending hat muss noch entschieden was mit ihr passiert.
Wenn die oberste Kette abgeschlossen wird und sie Pending ist wird sie und alle Subelemente verworfen
Wenn eine Subkette Pending hat wird sie vorerst behalten. Je nach entscheidung der übergeordneten Kette wird die Subkette behalten oder verworfen
Der Status Deleting löscht die Kette. Kommende Eventeinträge werden verworfen, der verweis auf diese Kette wird als Gelöscht markiert.
In der Konfiguration der übergeordneten kette steht wie weiter verfahren werden soll:
Es gibt in der übergeordnetten Kette 2 Möglichkeiten: Nur die betroffene Subkette löschen. Oder Selber (mit anderen Subketten löschen) und an die übergeordnete kette melden
Wenn eine Kette als Savin Markiert wird wird das an die übergeordnete kette gemeldet. Diese hat dan wieder 2 Möglichkeiten zu entscheiden.
z.B.: nicht beachten oder 

