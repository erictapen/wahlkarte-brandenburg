#! /usr/bin/env python3

from openpyxl import load_workbook
from collections import defaultdict

import json
import sys

def excel_column_name(n):
    """Number to Excel-style column name, e.g., 1 = A, 26 = Z, 27 = AA, 703 = AAA."""
    name = ""
    while n > 0:
        n, r = divmod (n - 1, 26)
        name = chr(r + ord('A')) + name
    return name

def generate_election_results(*,
        rawfile,
        outfile,
        sheetname="Ergebnis",
        human_readable_name,
        first_party_index,
        start_row=2,
        end_row,
        keys={
            "Landkreisnummer": "C",
            "AGS": "B",
            "Gültige Stimmen": "S"
        }
            ):
    workbook = load_workbook(filename = sys.argv[1] + rawfile)

    worksheet = workbook[sheetname]

    # determine all available parties
    parties = {}
    rowi = first_party_index
    while True:
        cell_value = worksheet[excel_column_name(rowi) + "1"].value
        if cell_value is not None:
            parties[cell_value] = excel_column_name(rowi)
            rowi = rowi + 1
        else:
            break

    print("Es gibt " + str(len(parties)) + " Parteien in " + rawfile + ".")

    # build a data structure containing all party names for this particular election.
    select_options.append( {
        "name": outfile.replace(".json", ""),
        "human_readable_name": human_readable_name,
        "parties": list(parties.keys())
        } )

    # prepare overall structure of result dict
    result = dict()
    result["_absolute"] = defaultdict(int)
    for party in parties:
        result[party] = defaultdict(int)

    # Iterate for every wahlbezirk for every party and aggregate results
    for line in range(start_row, end_row + 1):
        print(str(line) + ": " + str(worksheet["F" + str(line)].value))
        lk_nr = worksheet[keys["Landkreisnummer"] + str(line)].value
        try:
            # The AGS has sometimes two extra digits. We don't want them yet.
            ags = worksheet[keys["AGS"] + str(line)].value[:8]
        except:
            TypeError
            ags = str(worksheet[keys["AGS"] + str(line)].value)
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
    # This is useful for visualization as we don't have to calculate it in the browser.
    for party, value in result.items():
        highest_ratio = 0.0
        for ags, votes in result[party].items():
            highest_ratio = max(highest_ratio, votes / result['_absolute'][ags])
        result[party]["_highest_ratio"] = highest_ratio

    with open(sys.argv[2] + outfile, "w", encoding="utf-8") as out_file:
        json.dump(result, out_file, ensure_ascii=False, indent=4)

# Write the content of the select menu to file.
def write_select_options( select_options ):
    with open(sys.argv[2] + "select_options.json", "w", encoding="utf-8") as f:
        json.dump(select_options, f, ensure_ascii=False)

# Data structure that will contain all the contents for the select menu.
select_options = []

generate_election_results(
    rawfile="DL_BB_EU2019.xlsx",
    outfile = "eu2019.json",
    sheetname = "Brandenburg_Europawahl_W",
    human_readable_name = "Europawahl 2019",
    first_party_index = 20, # T
    end_row = 3812,
    keys = {
        "Landkreisnummer": "C",
        "AGS": "B",
        "Gültige Stimmen": "S"
        }
    )

generate_election_results(
    rawfile = "DL_BB_EU2014.xlsx",
    outfile = "eu2014.json",
    sheetname = "Ergebnis",
    human_readable_name = "Europawahl 2014",
    first_party_index = 18, # R
    end_row = 3679,
    keys = {
        "Landkreisnummer": "C",
        "AGS": "B",
        "Gültige Stimmen": "Q"
        }
    )

write_select_options(select_options)

