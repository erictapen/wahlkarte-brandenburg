#! /usr/bin/env python3

from openpyxl import load_workbook
from collections import defaultdict

import json
import sys

# first arg: the DL_BB_EU2019.xlsx file
# second arg: the JSON output path


workbook = load_workbook(filename = sys.argv[1])

worksheet = workbook["Brandenburg_Europawahl_W"]

keys = {
  "AGS": "B",
  "Landkreisnummer": "C",
  "Amtsnummer": "D",
  "Gemeindenummer": "E",
  "Gemeindename": "F",
  "Wahlbezirk": "G",
  "Wahlbezirksart": "H",
  "Wahlberechtigte insgesamt": "L",
  "Wähler": "P",
  "Gültige Stimmen": "S",
}

def excel_column_name(n):
    """Number to Excel-style column name, e.g., 1 = A, 26 = Z, 27 = AA, 703 = AAA."""
    name = ''
    while n > 0:
        n, r = divmod (n - 1, 26)
        name = chr(r + ord('A')) + name
    return name

parties = {}
rowi = 20 # "T" in excel rows
while True:
    cell_value = worksheet[excel_column_name(rowi) + "1"].value
    if cell_value is not None:
        parties[cell_value] = excel_column_name(rowi)
        rowi = rowi + 1
    else:
        break

print("Es gibt " + str(len(parties)) + " Parteien.")

result = dict()
result["_absolut"] = defaultdict(int)
for party in parties:
    result[party] = defaultdict(int)

for line in range(2, 3813):
    print(str(line) + ": " + str(worksheet["F" + str(line)].value))
    lk_nr = worksheet[keys["Landkreisnummer"] + str(line)].value
    # The AGS has sometimes two extra digits. We don't want them yet.
    ags = worksheet[keys["AGS"] + str(line)].value[:8]
    absolute_votes = worksheet[keys["Gültige Stimmen"] + str(line)].value
    # accumulate the valid votes for both the Landkreis and the Gemeinde
    result["_absolut"][lk_nr] += absolute_votes
    result["_absolut"][ags] += absolute_votes
    # print("lk: " + lk_nr + ", ags: " + ags)
    for (party, party_key) in parties.items():
        # accumulate both Landkreis wide and Gemeinde wide amounts of votes for the party
        result[party][lk_nr] += worksheet[party_key + str(line)].value
        result[party][ags] += worksheet[party_key + str(line)].value

with open(sys.argv[2], "w", encoding="utf-8") as out_file:
    json.dump(result, out_file, ensure_ascii=False, indent=4)
