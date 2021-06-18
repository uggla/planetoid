package fr.uggla;


import fr.uggla.Player;
import io.quarkus.hibernate.orm.panache.PanacheEntity;

import javax.persistence.*;
import java.time.LocalDate;
import java.util.Date;
import java.util.HashSet;
import java.util.Set;

@Entity
public class Game extends PanacheEntity {
    public Date gamedate;
    @ManyToMany(cascade = { CascadeType.ALL })
    @JoinTable(
            name = "Game_Player",
            joinColumns = { @JoinColumn(name = "game_id") },
            inverseJoinColumns = { @JoinColumn(name = "player_id") }
    )
    Set<Player> player = new HashSet<>();
}