import { invoke } from "@tauri-apps/api/core";

import type { Conversation } from "./conversation";
import type { MessageView } from "./message";

export function createConversation(
    title: string,
): Promise<Conversation> {
    return invoke<Conversation>("create_conversation", {
        request: {
            title,
        },
    });
}

export function listConversations(): Promise<Conversation[]> {
    return invoke<Conversation[]>("list_conversations");
}

export function sendMessage(
    conversationId: string,
    content: string,
): Promise<MessageView> {
    return invoke<MessageView>("send_message", {
        request: {
            conversationId,
            content,
        },
    });
}

export function loadMessages(
    conversationId: string,
): Promise<MessageView[]> {
    return invoke<MessageView[]>("load_messages", {
        request: {
            conversationId,
        },
    });
}