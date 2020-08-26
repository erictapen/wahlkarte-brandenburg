#! /usr/bin/env python3

from openpyxl import load_workbook
from collections import defaultdict

import json
import sys

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

def generate_election_results(config):
    workbook = load_workbook(filename = sys.argv[1] + config["rawfile"])

    worksheet = workbook["Brandenburg_Europawahl_W"]

    # determine all available parties
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

    # prepare overall structure of result dict
    result = dict()
    result["_absolute"] = defaultdict(int)
    for party in parties:
        result[party] = defaultdict(int)

    # Iterate for every wahlbezirk for every party and aggregate results
    for line in range(config["start_row"], config["end_row"]):
        print(str(line) + ": " + str(worksheet["F" + str(line)].value))
        lk_nr = worksheet[keys["Landkreisnummer"] + str(line)].value
        # The AGS has sometimes two extra digits. We don't want them yet.
        ags = worksheet[keys["AGS"] + str(line)].value[:8]
        absolute_votes = worksheet[keys["Gültige Stimmen"] + str(line)].value
        # accumulate the valid votes for both the Landkreis and the Gemeinde
        result["_absolute"][lk_nr] += absolute_votes
        result["_absolute"][ags] += absolute_votes
        # print("lk: " + lk_nr + ", ags: " + ags)
        for (party, party_key) in parties.items():
            # accumulate both Landkreis wide and Gemeinde wide amounts of votes for the party
            result[party][lk_nr] += worksheet[party_key + str(line)].value
            result[party][ags] += worksheet[party_key + str(line)].value

    # Determine what was the highest percentage the party ever gathered in a given Wahlbezirk.
    # This is useful for visualization so we don't have to compute it in the browser.
    for party, value in result.items():
        highest_ratio = 0.0
        for ags, votes in result[party].items():
            highest_ratio = max(highest_ratio, votes / result['_absolute'][ags])
        result[party]["_highest_ratio"] = highest_ratio

    with open(sys.argv[2] + config["outfile"], "w", encoding="utf-8") as out_file:
        json.dump(result, out_file, ensure_ascii=False, indent=4)

generate_election_results( {
    "rawfile": "DL_BB_EU2019.xlsx",
    "outfile": "eu2019.json",
    "start_row": 2,
    "end_row": 3813
    } )

