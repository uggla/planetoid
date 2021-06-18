package fr.uggla;

import io.quarkus.hibernate.orm.panache.PanacheQuery;

import javax.transaction.Transactional;
import javax.ws.rs.*;
import javax.ws.rs.core.MediaType;
import javax.ws.rs.core.Response;
import java.util.Date;
import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;

@Path("/games")
@Produces(MediaType.APPLICATION_JSON)
@Consumes(MediaType.APPLICATION_JSON)
public class Games {

    @GET
    public Set<Date> games() {
        List<Game> allGames = Game.listAll();
        return allGames.stream().map(game -> game.gamedate).collect(Collectors.toSet());
    }

    @Transactional
    @POST
    public Response add(Game truc) {
        try {
            Game game = new Game();
            game.gamedate = new Date();
            PanacheQuery<Player> player2 = Player.find("from Player where name='titi'");
            player2.page(io.quarkus.panache.common.Page.ofSize(25));
            List<Player> firstPage = player2.list();
            System.out.println(firstPage.get(0).name);
            game.player.add(firstPage.get(0));
            game.persist();
            return Response.ok(game).status(201).build();
        } catch (Exception e) {
            System.out.println(e);
            return Response.serverError().build();
        }
    }
}
