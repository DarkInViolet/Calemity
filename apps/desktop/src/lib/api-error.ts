export type ApiErrorCode =
    | "invalid_request"
    | "conversation_title_empty"
    | "conversation_title_too_long"
    | "conversation_not_found"
    | "message_content_empty"
    | "permission_denied"
    | "storage_failure"
    | "internal_failure";

export interface ApiError {
    code: ApiErrorCode;
    message: string;
}

function isApiError(error: unknown): error is ApiError {
    if (
        typeof error !== "object" ||
        error === null
    ) {
        return false;
    }

    const candidate = error as Record<string, unknown>;

    return (
        typeof candidate.code === "string" &&
        typeof candidate.message === "string"
    );
}

export function getErrorMessage(error: unknown): string {
    if (isApiError(error)) {
        return error.message;
    }

    if (error instanceof Error) {
        return error.message;
    }

    if (typeof error === "string") {
        return error;
    }

    return "An unexpected error occurred";
}