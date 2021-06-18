package fr.uggla;

import javax.transaction.Transactional;
import javax.ws.rs.*;
import javax.ws.rs.core.MediaType;
import javax.ws.rs.core.Response;
import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;

@Path("/players")
@Produces(MediaType.APPLICATION_JSON)
@Consumes(MediaType.APPLICATION_JSON)
public class Players {

    @GET
    public Set<String> players() {
        List<Player> allPlayers = Player.listAll();
        return allPlayers.stream().map(player -> player.name).collect(Collectors.toSet());
    }

    @Transactional
    @POST
    public Response add(Player player) {
        try {
            player.persist();
            return Response.ok(player).status(201).build();
        } catch (Exception e) {
            return Response.serverError().build();
        }
    }
}

