# Data Loading:
import pandas as pd
supply_data = pd.read_csv ("supply_chain_data.csv")
supply_data.head ()
supply_data.columns

# Check for missing values
missing_values = supply_data.isnull().sum()
supply_data.info()

# Display columns with missing values and the count of missing values
missing_values = missing_values[missing_values > 0]

if not missing_values.empty:
    print("Columns with missing values:")
    for column, count in missing_values.items():
        print(f"{column}: {count} missing values")
else:
    print("There are no columns with missing value")
#Check for duplicate data
if supply_data.duplicated().any():
    print(f"There are as many as {supply_data.duplicated().sum()} duplicate data.")
else:
    print("There are no duplicate data.")