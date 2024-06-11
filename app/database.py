from neo4j import GraphDatabase

class Neo4jConnection:
    def __init__(self, uri, user, pwd):
        self.__uri = uri
        self.__user = user
        self.__pwd = pwd
        self.__driver = None
        try:
            self.__driver = GraphDatabase.driver(self.__uri, auth=(self.__user, self.__pwd))
            print("Neo4j connection initialized.")
        except Exception as e:
            print("Failed to create the driver:", e)


    def close(self):
        if self.__driver is not None:
            self.__driver.close()

    def query(self, query, parameters=None, db=None):
        assert self.__driver is not None, "Driver not initialized!"
        try:
            with self.__driver.session(database=db) as session:
                result = session.run(query, parameters)
                return [record.data() for record in result]
        except Exception as e:
            print("Query failed:", e)
            return []


    def get_statistics(self):
        query = """
        MATCH (n) RETURN count(n) AS TotalNodes;
        """
        return self.query(query)

    def get_recommendations(self, user_id):
        query = """
        MATCH (t:Transporter)-[DELIVERS]->(c:Customer)
        MATCH (s:Supplier)-[SHIPS]->(t)
        WHERE NOT (t)-[:SHIPS]->(:Supplier) // Exclude transporters delivering to suppliers
        MATCH (s)-[:LOCATED_IN]->(supplierLocation:Location)
        MATCH (c)-[:LOCATED_IN]->(customerLocation:Location)
        WHERE supplierLocation <> customerLocation // Ensure supplier is in a different location from the customer
        RETURN s, t, c, SHIPS.shippingCosts AS shippingCosts
        ORDER BY shippingCosts ASC // Order by shipping costs for cost-effective routes
        LIMIT 10;
        """
        return self.query(query, parameters={"userId": user_id})

    def get_centrality(self):
        query = """
       MATCH path = (:Supplier)-[:SUPPLIES]->(:Product)<-[:PURCHASES]-(:Customer) RETURN path LIMIT 10;
        """
        return self.query(query)

    def get_path_analysis(self):
        query = """
   MATCH (c:Customer)-[r:PURCHASES]->(p:Product) 
   WITH p, sum(toInteger(r.numberSold)) AS totalSold
   RETURN p.productType, p.sku, totalSold 
   ORDER BY totalSold DESC; 
        """
        return self.query(query)

    def get_community_detection(self):
        query = """
        
        """
        return self.query(query)
