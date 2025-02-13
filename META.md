# Project meta data

## Register structure

### AKM (Labor Market Status)

Collected annually from 2000 to 2022
Contains the following columns:

"PNR": String, (required)
"SOCIO": Integer (possibly missing)
"SOCIO02": Integer (possibly missing)
"SOCIO13": Integer (required) # Socioeconomic classification
"CPRTJEK": Integer (possibly missing)
"CPRTYPE": Integer (possibly missing)
"VERSION": String (possibly missing)
"SENR": String (possibly missing)

### BEF (Population)

Collected annually from 2000 to 2018
Then after every quarter from 2019 to 2023
Contains the following columns:

"AEGTE_ID": String, # PNR of the spouse
"ALDER": String, # Age
"ANTBOERNF": Integer, # Number of children in the family
"ANTBOERNH": Integer, # Number of children in the household
"ANTPERSF": Integer, # Number of persons in the family
"ANTPERSH": Integer, # Number of persons in the household
"BOP_VFRA": Date, # Date of moving in
"CIVST": String, # Civil status
"CPRTJEK": Integer, # Checksum for CPR/PNR number
"CPRTYPE": Integer, # Type of CPR/PNR number
"E_FAELLE_ID": String, # PNR of the partner
"FAMILIE_ID": String, # Unique family ID
"FAMILIE_TYPE": String, # Family type
"FAR_ID": String, # PNR of the father
"FM_MARK": String, # Family mark
"FOED_DAG": Date, # Date of birth
"HUSTYPE": String, # Household type
"IE_TYPE": String, # Immigration/emigration type
"KOEN": String, # Gender
"KOM": Integer, # Municipality code
"MOR_ID": String, # PNR of the mother
"OPR_LAND": String, # Country of origin
"PLADS": String, # The person's place in the family
"PNR": String, # CPR/PNR number
"REG": String, # Region
"STATSB": String, # Citizenship
"VERSION": String, # Version of the data

### IND (Income)

Collected annually from 2000 to 2022
Contains the following columns:

"BESKST13": Integer, # Code for person's main source of income
"CPRTJEK": String,
"CPRTYPE": String,
"LOENMV_13": Float, # Total wage income
"PERINDKIALT_13": Float, # Total personal income excl. calculated rental value of own home
"PNR": String,
"PRE_SOCIO": Integer,
"VERSION": String,

### UDDF (Education)

Only collected in 2020 and 2022
Contains the following columns:

"PNR": String,
"CPRTJEK": String,
"CPRTYPE": String,
"HFAUDD": String, # Highest completed education
"HF_KILDE": String,
"HF_VFRA": Date, # Time of highest completed education
"HF_VTIL": Date,
"INSTNR": Integer,
"VERSION": String,

## Storage structure

The registers are stored in the following way:

registers/ (root)
├── akm/ (register)
│ ├── 2000.parquet
...
│ └── 2022.parquet
├── bef/ (register )
│ ├── 200012.parquet
...
│ ├── 201903.parquet
...
│ └── 202309.parquet

├── ind/ (register)
│ ├── 2000.parquet
...
│ └── 2022.parquet

├── uddf/ (register)
│ ├── 202009.parquet
│ └── 202209.parquet
