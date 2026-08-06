import { useEffect, useState } from "react";
import { ChatHeader } from "./components/chat/ChatHeader";
import { ConversationSidebar } from "./components/chat/ConversationSidebar";
import { MessageComposer } from "./components/chat/MessageComposer";
import { MessageList } from "./components/chat/MessageList";
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
    const [conversations, setConversations] =
        useState<Conversation[]>([]);

    const [selectedConversationId, setSelectedConversationId] =
        useState<string | null>(null);

    const [messages, setMessages] = useState<Message[]>([]);
    const [messageText, setMessageText] = useState("");
    const [conversationTitle, setConversationTitle] = useState("");
    const [error, setError] = useState<string | null>(null);
    async function refreshConversations() {
        try {
            const loadedConversations =
                await listConversations();

            setConversations(loadedConversations);

            setSelectedConversationId((currentId) => {
                if (
                    currentId &&
                    loadedConversations.some(
                        (conversation) =>
                            conversation.id === currentId,
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
    async function refreshMessages(
        conversationId: string,
    ) {
        try {
            const loadedMessages =
                await loadMessages(conversationId);

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
            const conversation =
                await createConversation(title);

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

    const selectedConversation =
        conversations.find(
            (conversation) =>
                conversation.id === selectedConversationId,
        ) ?? null;

    return (
        <div className="app-shell">
            <ConversationSidebar
                conversations={conversations}
                selectedConversationId={selectedConversationId}
                conversationTitle={conversationTitle}
                onConversationTitleChange={setConversationTitle}
                onCreateConversation={handleCreateConversation}
                onSelectConversation={setSelectedConversationId}
            />

            <main className="chat-panel">
                <ChatHeader
                    conversation={selectedConversation}
                />

                {error && (
                    <div
                        className="error-banner"
                        role="alert"
                    >
                        {error}
                    </div>
                )}

                <MessageList
                    conversation={selectedConversation}
                    messages={messages}
                    currentAuthorId={AUTHOR}
                />

                <MessageComposer
                    value={messageText}
                    conversationTitle={
                        selectedConversation?.title ?? null
                    }
                    disabled={!selectedConversation}
                    onChange={setMessageText}
                    onSend={handleSendMessage}
                />
            </main>
        </div>
    );
}