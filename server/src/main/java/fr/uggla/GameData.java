package fr.uggla;

import java.nio.ByteBuffer;
import java.nio.charset.StandardCharsets;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

import javax.enterprise.context.ApplicationScoped;
import javax.websocket.OnClose;
import javax.websocket.OnError;
import javax.websocket.OnMessage;
import javax.websocket.OnOpen;
import javax.websocket.server.PathParam;
import javax.websocket.server.ServerEndpoint;
import javax.websocket.Session;

@ServerEndpoint("/gamedata/{username}")
@ApplicationScoped
public class GameData {

    Map<String, Session> sessions = new ConcurrentHashMap<>();

    @OnOpen
    public void onOpen(Session session, @PathParam("username") String username) {
        sessions.put(username, session);
        broadcast("User " + username + " joined");
    }

    @OnClose
    public void onClose(Session session, @PathParam("username") String username) {
        sessions.remove(username);
    }

    @OnError
    public void onError(Session session, @PathParam("username") String username, Throwable throwable) {
        sessions.remove(username);
        broadcast("User " + username + " left on error: " + throwable);
    }

// Example using binary message
//    @OnMessage
//    public void onMessage(ByteBuffer message, @PathParam("username") String username) {
//        String msg = StandardCharsets.UTF_8.decode(message).toString();
//        System.out.println("Received msg:" + msg );
//        broadcast(">> " + username + ": " + msg);
//     }
    @OnMessage
    public void onMessage(String message, @PathParam("username") String username) {
        System.out.println("Received msg:" + message);
//        broadcast(">> " + username + ": " + message);
        broadcast(message);
    }

    private void broadcast(String message) {
        sessions.values().forEach(s -> {
            s.getAsyncRemote().sendObject(message, result -> {
                if (result.getException() != null) {
                    System.out.println("Unable to send message: " + result.getException());
                }
            });
        });
    }
}