GRAPH STATISTICS:

1. Total Number of Nodes:

   MATCH (n) RETURN count(n) AS TotalNodes;

   Description: Returns the total number of nodes in the supply chain graph.

2. Number of Isolated Nodes:
   
   MATCH (n)
   WHERE NOT ()--(n)
   RETURN count(n) AS IsolatedNodes;
   
   Description: Counts the number of nodes in the supply chain graph that are not connected to any other node.

3. Total Number of Suppliers:
   
   MATCH (s:Supplier) RETURN count(s) AS TotalSuppliers;
   
   Description: Retrieves the total number of supplier nodes in the supply chain graph.

4. Total Number of Customers:
   
   MATCH (c:Customer) RETURN count(c) AS TotalCustomers;
   
   Description: Retrieves the total number of customer nodes in the supply chain graph.

5. Total Number of Products:
   
   MATCH (p:Product) RETURN count(p) AS TotalProducts;
   
   Description: Retrieves the total number of product nodes in the supply chain graph.

6. Number of Supplier-Product Relationships:
   
   MATCH (:Supplier)-[r:SUPPLIES]->(:Product) RETURN count(r) AS SupplierProductRelationships;
   
   Description: Counts the number of relationships representing the supply of products by suppliers in the supply chain graph.

7. Number of Customer-Product Relationships:
   
   MATCH (:Customer)-[r:PURCHASES]->(:Product) RETURN count(r) AS CustomerProductRelationships;
   
   Description: Counts the number of relationships representing the purchase of products by customers in the supply chain graph.

8. Total Revenue Generated:
   
   MATCH (:Customer)-[r:PURCHASES]->(p:Product) RETURN sum(p.price) AS TotalRevenue;
   
   Description: Calculates the total revenue generated from product purchases by customers in the supply chain graph.

9. Average Price of Products:
   
   MATCH (p:Product) RETURN avg(p.price) AS AveragePrice;
   
   Description: Computes the average price of products across all nodes in the supply chain graph.

10. Total Number of Relationships:
    
    MATCH ()-[r]->() RETURN count(r) AS TotalRelationships;
    
    Description: Counts the total number of relationships (edges) in the supply chain graph, regardless of their type.


GRAPH ANALYTICS:

1. Path Analysis:

   MATCH path = (:Supplier)-[:SUPPLIES]->(:Product)<-[:PURCHASES]-(:Customer)
   RETURN path
   LIMIT 10;

   Description: Finds paths between suppliers and customers in the supply chain graph, representing the flow of products from suppliers to customers

2. Node Interaction Analysis:

   MATCH (s:Supplier)-[r:SUPPLIES]->(:Product)<-[r2:PURCHASES]-(:Customer)
   WHERE s.supplierName = 'SupplierA' // Update with the supplier of interest
   RETURN s.supplierName, count(r2) AS InteractionCount
   ORDER BY InteractionCount DESC
   LIMIT 10;
   
   Description: Determines the number of interactions (product supplies and purchases) initiated by a specific supplier, helping to understand their engagement level with customers.

3. Identify high-demand products based on sales

   MATCH (c:Customer)-[r:PURCHASES]->(p:Product) 
   WITH p, sum(toInteger(r.numberSold)) AS totalSold
   RETURN p.productType, p.sku, totalSold 
   ORDER BY totalSold DESC; 

4. Find suppliers with high manufacturing capacities 
   
   MATCH (s:Supplier) RETURN s.supplierName, s.productionVolumes  
   ORDER BY s.productionVolumes DESC; 

5. Identify transportation carriers with specific modes

   MATCH (t:Transporter) 
   WHERE t.transportMode IN ['Air', 'Road','Rail'] 
   RETURN t.carrier, t.transportMode; 

6. Graph Statistics - Degree Distribution:

   MATCH (s:Supplier)-[:SUPPLIES]->()
   RETURN s.supplierName AS Supplier, count(*) AS Degree
   ORDER BY Degree DESC
   LIMIT 10;
   
   Description: Analyzes the degree distribution of suppliers, providing insights into their connectivity and relationships within the supply chain network.

- Link Prediction (Almost Implemented Some Bugs)
// Create a projected graph containing only Supplier and Customer nodes
CALL gds.graph.create('supplyChainGraph', ['Supplier', 'Customer'], {
  SUPPLIES: {
    type: 'SUPPLIES',
    orientation: 'UNDIRECTED'
  },
  PURCHASES: {
    type: 'PURCHASES',
    orientation: 'UNDIRECTED'
  }
});

// Compute Node Similarity for link prediction
CALL gds.nodeSimilarity.write('supplyChainGraph', {
  writeRelationshipType: 'SIMILARITY',
  similarityCutoff: 0.1
});

// Top predicted links based on Node Similarity
MATCH (s1)-[:SIMILARITY]->(s2)
WHERE id(s1) < id(s2)
RETURN s1, s2
ORDER BY gds.util.asNode(s1).supplierName, gds.util.asNode(s2).supplierName;


- Link Prediction for New Optimized Shipping Routes ( Working )
MATCH (t:Transporter)-[DELIVERS]->(c:Customer)
MATCH (s:Supplier)-[SHIPS]->(t)
WHERE NOT (t)-[:SHIPS]->(:Supplier) // Exclude transporters delivering to suppliers
MATCH (s)-[:LOCATED_IN]->(supplierLocation:Location)
MATCH (c)-[:LOCATED_IN]->(customerLocation:Location)
WHERE supplierLocation <> customerLocation // Ensure supplier is in a different location from the customer
RETURN s, t, c, SHIPS.shippingCosts AS shippingCosts 
ORDER BY shippingCosts ASC // Order by shipping costs for cost-effective routes
LIMIT 10;
- Some Betterments : 
MATCH (s:Supplier), (p:Product), (c:Customer), (t:Transporter)
WHERE s.stockLevels > 50  // Ensure supplier has sufficient stock levels
AND c.numberSold > 100    // Consider customers with high purchase volumes
AND t.shippingCosts < 500 // Prioritize transporters with lower shipping costs
RETURN s.supplierName AS Supplier, p.productType AS Product, c.customerid AS Customer, t.carrier AS Transporter, t.shippingCosts AS ShippingCosts
LIMIT 10; // Limit to top 10 predictions
