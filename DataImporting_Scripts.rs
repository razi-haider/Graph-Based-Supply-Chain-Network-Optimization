// Create Supplier nodes
LOAD CSV WITH HEADERS FROM 'file:///supply_chain_data.csv' AS row
MERGE (s:Supplier {supplierName: row.`Supplier name`})
ON CREATE SET s.location = row.Location;

// Create Customer nodes
LOAD CSV WITH HEADERS FROM 'file:///supply_chain_data.csv' AS row
MERGE (c:Customer {demographics: row.`Customer demographics`});

// Create Transporter nodes
LOAD CSV WITH HEADERS FROM 'file:///supply_chain_data.csv' AS row
MERGE (t:Transporter {carrier: row.`Shipping carriers`, transportMode: row.`Transportation modes`, route: row.Routes});

// Create Location nodes
LOAD CSV WITH HEADERS FROM 'file:///supply_chain_data.csv' AS row
MERGE (l:Location {name: row.Location});

///////////////////////////////////////

// Create relationships
LOAD CSV WITH HEADERS FROM 'file:///supply_chain_data.csv' AS row

// SUPPLIES relationship
MATCH (s:Supplier {supplierName: row.`Supplier name`}), (p:Product {sku: row.SKU})
MERGE (s)-[:SUPPLIES]->(p)
ON CREATE SET
    s.availability = toInteger(row.Availability),
    s.stockLevels = toInteger(row.`Stock levels`),
    s.leadTimes = toInteger(row.`Lead times`),
    s.productionVolumes = toInteger(row.`Production volumes`),
    s.manufacturingLeadTime = toInteger(row.`Manufacturing lead time`),
    s.manufacturingCosts = toFloat(row.`Manufacturing costs`);

// SHIPS relationship
MATCH (s:Supplier {supplierName: row.`Supplier name`}), (t:Transporter {carrier: row.`Shipping carriers`})
MERGE (s)-[:SHIPS]->(t)
ON CREATE SET
    s.orderQuantities = toInteger(row.`Order quantities`),
    s.shippingTimes = toInteger(row.`Shipping times`),
    s.shippingCosts = toFloat(row.`Shipping costs`);

// DELIVERS relationship
MATCH (t:Transporter {carrier: row.`Shipping carriers`}), (c:Customer {demographics: row.`Customer demographics`})
MERGE (t)-[:DELIVERS]->(c)
ON CREATE SET t.costs = toFloat(row.Costs);

// PURCHASES relationship
MATCH (c:Customer {demographics: row.`Customer demographics`}), (p:Product {sku: row.SKU})
MERGE (c)-[:PURCHASES]->(p)
ON CREATE SET
    c.numberSold = toInteger(row.`Number of products sold`),
    c.revenueGenerated = toFloat(row.`Revenue generated`),
    c.inspectionResults = row.`Inspection results`,
    c.defectRates = toFloat(row.`Defect rates`);

// LOCATED_IN relationship
MATCH (s:Supplier {supplierName: row.`Supplier name`}), (c:Customer {demographics: row.`Customer demographics`}), (t:Transporter {carrier: row.`Shipping carriers`}), (l:Location {name: row.Location})
MERGE (s)-[:LOCATED_IN]->(l)
MERGE (c)-[:LOCATED_IN]->(l)
MERGE (t)-[:LOCATED_IN]->(l);

// Unique constraint for Product SKU
CREATE CONSTRAINT ON (p:Product) ASSERT (p.sku) IS UNIQUE;

// Unique constraint for Supplier name
CREATE CONSTRAINT ON (s:Supplier) ASSERT (s.supplierName) IS UNIQUE;

// Existence constraint for Customer demographics
CREATE CONSTRAINT ON (c:Customer) ASSERT exists(c.demographics);

// Existence constraint for Transporter carrier
CREATE CONSTRAINT ON (t:Transporter) ASSERT exists(t.carrier);

// Node key constraint for Location
CREATE CONSTRAINT ON (l:Location) ASSERT (l.name, l.type) IS NODE KEY;
