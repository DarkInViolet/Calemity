import { invoke } from "@tauri-apps/api/core";

import type { Conversation } from "./conversation";
import type { Message } from "./message";

export function createConversation(
    title: string,
): Promise<Conversation> {
    return invoke<Conversation>("create_conversation", {
        title,
    });
}

export function listConversations(): Promise<Conversation[]> {
    return invoke<Conversation[]>("list_conversations");
}

export function sendMessage(
    authorId: string,
    conversationId: string,
    deviceId: string,
    content: string,
): Promise<void> {
    return invoke<void>("send_message", {
        authorId,
        conversationId,
        deviceId,
        content,
    });
}

export function loadMessages(
    conversationId: string,
): Promise<Message[]> {
    return invoke<Message[]>("load_messages", {
        conversationId,
    });
}