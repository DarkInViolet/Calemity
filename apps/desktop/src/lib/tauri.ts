import { invoke } from "@tauri-apps/api/core";

export async function sendMessage(
    authorId: string,
    conversationId: string,
    deviceId: string,
    content: string
) {
    return invoke("send_message", {
        authorId,
        conversationId,
        deviceId,
        content,
    });
}

export async function loadMessages(
    conversationId: string
) {
    return invoke("load_messages", {
        conversationId,
    });
}
