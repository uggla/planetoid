package fr.uggla;


import io.quarkus.hibernate.orm.panache.PanacheEntity;

import javax.persistence.Entity;
import javax.persistence.ManyToMany;
import javax.validation.constraints.NotBlank;
import java.util.HashSet;
import java.util.Set;

@Entity
public class Player extends PanacheEntity {
    @NotBlank
    public String name;
    @ManyToMany(mappedBy = "player")
    private Set<Game> game = new HashSet<>();
}