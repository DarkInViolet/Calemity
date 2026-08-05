export interface Message {
    id: string;
    author_id: string;
    conversation_id: string;
    device_id: string;
    content: string;
    timestamp: number;
}