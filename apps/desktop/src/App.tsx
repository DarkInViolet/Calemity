import { useEffect, useState } from "react";

import type { Conversation } from "./lib/conversation";
import type { Message } from "./lib/message";
import {
    createConversation,
    listConversations,
    loadMessages,
    sendMessage,
} from "./lib/tauri";

const AUTHOR = "01K1GK79HVM0R8Y8B5XJXQ6A1A";
const DEVICE_ID = "desktop";

export default function App() {
    const [conversations, setConversations] = useState<Conversation[]>([]);
    const [selectedConversationId, setSelectedConversationId] =
        useState<string | null>(null);

    const [messages, setMessages] = useState<Message[]>([]);
    const [messageText, setMessageText] = useState("");
    const [conversationTitle, setConversationTitle] = useState("");
    const [error, setError] = useState<string | null>(null);

    async function refreshConversations() {
        try {
            const loadedConversations = await listConversations();

            setConversations(loadedConversations);

            setSelectedConversationId((currentId) => {
                if (
                    currentId &&
                    loadedConversations.some(
                        (conversation) => conversation.id === currentId,
                    )
                ) {
                    return currentId;
                }

                return loadedConversations[0]?.id ?? null;
            });

            setError(null);
        } catch (caughtError) {
            setError(String(caughtError));
        }
    }

    async function refreshMessages(conversationId: string) {
        try {
            const loadedMessages = await loadMessages(conversationId);

            setMessages(loadedMessages);
            setError(null);
        } catch (caughtError) {
            setError(String(caughtError));
        }
    }

    async function handleCreateConversation() {
        const title = conversationTitle.trim();

        if (!title) {
            return;
        }

        try {
            const conversation = await createConversation(title);

            setConversationTitle("");
            await refreshConversations();
            setSelectedConversationId(conversation.id);
            setError(null);
        } catch (caughtError) {
            setError(String(caughtError));
        }
    }

    async function handleSendMessage() {
        const content = messageText.trim();

        if (!content || !selectedConversationId) {
            return;
        }

        try {
            await sendMessage(
                AUTHOR,
                selectedConversationId,
                DEVICE_ID,
                content,
            );

            setMessageText("");
            await refreshMessages(selectedConversationId);
            setError(null);
        } catch (caughtError) {
            setError(String(caughtError));
        }
    }

    useEffect(() => {
        void refreshConversations();
    }, []);

    useEffect(() => {
        if (!selectedConversationId) {
            setMessages([]);
            return;
        }

        void refreshMessages(selectedConversationId);
    }, [selectedConversationId]);

    const selectedConversation = conversations.find(
        (conversation) => conversation.id === selectedConversationId,
    );

    return (
        <div
            style={{
                display: "grid",
                gridTemplateColumns: "240px minmax(0, 1fr)",
                height: "100vh",
                fontFamily: "sans-serif",
            }}
        >
            <aside
                style={{
                    borderRight: "1px solid #444",
                    padding: 16,
                    overflowY: "auto",
                }}
            >
                <h1 style={{ marginTop: 0 }}>Calemity</h1>

                <div
                    style={{
                        display: "flex",
                        gap: 8,
                        marginBottom: 20,
                    }}
                >
                    <input
                        value={conversationTitle}
                        placeholder="New conversation"
                        onChange={(event) =>
                            setConversationTitle(event.target.value)
                        }
                        onKeyDown={(event) => {
                            if (event.key === "Enter") {
                                void handleCreateConversation();
                            }
                        }}
                        style={{
                            minWidth: 0,
                            width: "100%",
                        }}
                    />

                    <button
                        type="button"
                        onClick={() => void handleCreateConversation()}
                    >
                        +
                    </button>
                </div>

                {conversations.length === 0 ? (
                    <p>No conversations yet.</p>
                ) : (
                    conversations.map((conversation) => (
                        <button
                            key={conversation.id}
                            type="button"
                            onClick={() =>
                                setSelectedConversationId(conversation.id)
                            }
                            style={{
                                display: "block",
                                width: "100%",
                                padding: 10,
                                marginBottom: 8,
                                textAlign: "left",
                                fontWeight:
                                    conversation.id === selectedConversationId
                                        ? "bold"
                                        : "normal",
                            }}
                        >
                            {conversation.title}
                        </button>
                    ))
                )}
            </aside>

            <main
                style={{
                    display: "grid",
                    gridTemplateRows: "auto minmax(0, 1fr) auto",
                    padding: 20,
                    minWidth: 0,
                }}
            >
                <header>
                    <h2>
                        {selectedConversation?.title ??
                            "Select or create a conversation"}
                    </h2>

                    {error && (
                        <p style={{ fontWeight: "bold" }}>
                            {error}
                        </p>
                    )}
                </header>

                <section
                    style={{
                        border: "1px solid #444",
                        padding: 12,
                        overflowY: "auto",
                    }}
                >
                    {!selectedConversation ? (
                        <p>Create a conversation to begin.</p>
                    ) : messages.length === 0 ? (
                        <p>No messages yet.</p>
                    ) : (
                        messages.map((message) => (
                            <div
                                key={message.id}
                                style={{
                                    marginBottom: 10,
                                }}
                            >
                                {message.content}
                            </div>
                        ))
                    )}
                </section>

                <footer
                    style={{
                        display: "flex",
                        gap: 8,
                        paddingTop: 12,
                    }}
                >
                    <input
                        value={messageText}
                        placeholder={
                            selectedConversation
                                ? "Write a message"
                                : "Select a conversation first"
                        }
                        disabled={!selectedConversation}
                        onChange={(event) =>
                            setMessageText(event.target.value)
                        }
                        onKeyDown={(event) => {
                            if (event.key === "Enter") {
                                void handleSendMessage();
                            }
                        }}
                        style={{
                            flex: 1,
                        }}
                    />

                    <button
                        type="button"
                        disabled={!selectedConversation}
                        onClick={() => void handleSendMessage()}
                    >
                        Send
                    </button>
                </footer>
            </main>
        </div>
    );
}