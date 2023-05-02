"""
codegen.py

  Generate entries for a custom macro for the elements of the periodic table.
"""

import json

__author__ = """R2Boyo25 (KazaniAvali) & Pin Lee - (C) 2023"""


def field(a, key):
    "Get a field from every dict in a list"

    return map(lambda x: x[1][key], a.items())


def longest(a):
    "Get the length of the longest string"

    return max(map(len, a))


if __name__ == "__main__":
    with open("./element_data/elements_by_number.json", "r", encoding='utf-8') as f:
        data = json.load(f)

    lname = longest(field(data, "element_name"))
    lmass = longest(field(data, "atomic_mass"))
    lmasswhole = longest(map(lambda x: x.split(".")[0], field(data, "atomic_mass")))
    lmassfract = longest(
        map(lambda x: x.split(".")[1] if "." in x else "", field(data, "atomic_mass"))
    )

    with open("generated.rs", "w", encoding='utf-8') as generated:
        generated.write(f"build_elements! {{{len(data)} ELEMENTS;\n")

        for i, (key, value) in enumerate(data.items()):
            mass = value["atomic_mass"]

            padded_mass = (
                (
                    mass.split(".")[0].rjust(lmasswhole, " ")
                    + "."
                    + mass.split(".")[1].ljust(lmassfract, "0")
                )
                if "." in mass
                else (mass.rjust(lmasswhole, " ") + "." + "0" * lmassfract)
            )

            generated.write(
                f"    {value['atomic_symbol'].ljust(2)}"
                f" {value['atomic_number']}:{' '*(3-len(value['atomic_number']))}"
                f" {value['element_name'].ljust(lname)}"
                f" {padded_mass}"
                f" \"{value['electron_configuration']}\"{',' if i < len(data) - 1 else ''}\n"
                    .replace(" (calculated)", "")
                    .replace(" (predicted)", "")
            )

        generated.write("}")

    print(f"Generated entries for {len(data)} elements!")
