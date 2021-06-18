package fr.uggla;

import io.quarkus.test.junit.QuarkusTest;
import org.junit.jupiter.api.Test;

import static io.restassured.RestAssured.given;
import static org.hamcrest.CoreMatchers.is;

@QuarkusTest
public class PlayersTest {

    @Test
    public void testPlayersEndpoint() {
        given()
          .when().get("/players")
          .then()
             .statusCode(200)
             .body(is("[\"Uggla\",\"Rose\",\"Planetoid\"]"));
    }

}