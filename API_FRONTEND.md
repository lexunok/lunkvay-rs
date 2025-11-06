# Документация API для фронтенда

Этот документ описывает API-интерфейсы, предоставляемые LunkvayAPI, для использования фронтенд-приложениями.

## Базовый URL

`/api/v1`

## Аутентификация

Большинство конечных точек API требуют аутентификации с использованием токена JWT (JSON Web Token). После успешного входа в систему (через `/api/v1/Auth/login`) вы получите токен JWT. Этот токен должен быть включен в заголовок `Authorization` всех последующих запросов в формате `Bearer <токен>`. 

Пример заголовка:

```
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

## Общие типы данных

### UserDTO

Представляет информацию о пользователе.

```typescript
interface UserDTO {
  id: string; // GUID
  userName: string;
  email?: string;
  firstName?: string;
  lastName?: string;
  createdAt?: string; // ISO 8601 date string
  isDeleted?: boolean;
  lastLogin?: string; // ISO 8601 date string
  isOnline?: boolean;
}
```

### UserListItemDTO

Представляет краткую информацию о пользователе, используемую в списках.

```typescript
interface UserListItemDTO {
  userId: string; // GUID
  firstName: string;
  lastName?: string;
  isOnline?: boolean;
}
```

### ChatMessageDTO

Представляет сообщение чата.

```typescript
interface ChatMessageDTO {
  id: string; // GUID
  sender?: UserDTO;
  systemMessageType: SystemMessageType;
  message: string;
  isEdited?: boolean;
  isPinned?: boolean;
  createdAt?: string; // ISO 8601 date string
  updatedAt?: string; // ISO 8601 date string
  isMyMessage?: boolean;
}
```

### ChatMemberDTO

Представляет участника чата.

```typescript
interface ChatMemberDTO {
  id: string; // GUID
  member?: UserDTO;
  memberName?: string;
  role: ChatMemberRole;
}
```

### ChatDTO

Представляет информацию о чате.

```typescript
interface ChatDTO {
  id: string; // GUID
  name?: string;
  lastMessage?: ChatMessageDTO;
  type: ChatType;
  createdAt: string; // ISO 8601 date string
  memberCount: number;
}
```

### FriendshipDTO

Представляет информацию о дружбе.

```typescript
interface FriendshipDTO {
  friendshipId: string; // GUID
  status?: FriendshipStatus;
  labels?: FriendshipLabelDTO[];
  userId: string; // GUID
  firstName?: string;
  lastName?: string;
  isOnline?: boolean;
}
```

### FriendshipLabelDTO

Представляет метку дружбы.

```typescript
interface FriendshipLabelDTO {
  id: string; // GUID
  label: string;
}
```

### ProfileDTO

Представляет профиль пользователя.

```typescript
interface ProfileDTO {
  id: string; // GUID
  user: UserDTO;
  status?: string;
  about?: string;
  friendsCount?: number;
  friends?: UserListItemDTO[];
}
```

### Перечисления

#### ChatType

Тип чата.

*   `Personal`
*   `Group`

#### ChatMemberRole

Роль участника чата.

*   `Member`
*   `Administrator`
*   `Owner`

#### FriendshipStatus

Статус дружбы.

*   `Pending` (Ожидание)
*   `Accepted` (Подтверждено)
*   `Rejected` (Отклонено)
*   `Cancelled` (Отменено)

#### SystemMessageType

Тип системного сообщения в чате.

*   `None`
*   `UserJoined`
*   `UserRejoined`
*   `UserLeft`
*   `ChatCreated`
*   `ChatUpdated`

## Конечные точки API

---

### Аутентификация (`/Auth`)

#### `POST /api/v1/Auth/login`

Вход пользователя в систему.

*   **Аутентификация:** Не требуется.
*   **Тело запроса:**
    ```typescript
    interface LoginRequest {
      email: string;
      password: string;
    }
    ```
*   **Ответ:**
    *   `200 OK`: Строка JWT-токена.
    *   `400 Bad Request`, `401 Unauthorized`: Сообщение об ошибке (строка).

#### `POST /api/v1/Auth/logout`

Выход пользователя из системы (обрабатывается на стороне клиента).

*   **Аутентификация:** Не требуется.
*   **Тело запроса:** Нет.
*   **Ответ:**
    *   `200 OK`.

#### `POST /api/v1/Auth/register`

Регистрация нового пользователя.

*   **Аутентификация:** Не требуется.
*   **Тело запроса:**
    ```typescript
    interface RegisterRequest {
      email: string;
      userName: string;
      password: string;
      firstName?: string;
      lastName?: string;
    }
    ```
*   **Ответ:**
    *   `200 OK`: Успешная регистрация.
    *   `400 Bad Request`, `409 Conflict`: Сообщение об ошибке (строка).

---

### Аватары (`/Avatar`)

#### `GET /api/v1/Avatar`

Получает аватар аутентифицированного пользователя.

*   **Аутентификация:** Требуется.
*   **Запрос:** Нет.
*   **Ответ:**
    *   `200 OK`: Файл изображения (`image/webp`).
    *   `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `GET /api/v1/Avatar/{userId}`

Получает аватар пользователя по ID.

*   **Аутентификация:** Не требуется.
*   **Параметры пути:**
    *   `userId`: `string` (GUID) - ID пользователя.
*   **Ответ:**
    *   `200 OK`: Файл изображения (`image/webp`).
    *   `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `POST /api/v1/Avatar`

Загружает новый аватар для аутентифицированного пользователя.

*   **Аутентификация:** Требуется.
*   **Тело запроса:** `multipart/form-data` с полем `avatarFile` (файл изображения).
    *   Максимальный размер файла: 5 МБ.
    *   Разрешенные расширения: `.jpg`, `.jpeg`, `.png`, `.gif`, `.bmp`.
*   **Ответ:**
    *   `200 OK`: Файл изображения (`image/webp`), представляющий загруженный и обработанный аватар.
    *   `400 Bad Request`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `DELETE /api/v1/Avatar`

Удаляет аватар аутентифицированного пользователя.

*   **Аутентификация:** Требуется.
*   **Запрос:** Нет.
*   **Ответ:**
    *   `200 OK`: Успешное удаление.
    *   `500 Internal Server Error`: Сообщение об ошибке (строка).

---

### Пользователи (`/User`)

#### `GET /api/v1/User/{userId}`

Получает информацию о пользователе по ID.

*   **Аутентификация:** Требуется.
*   **Параметры пути:**
    *   `userId`: `string` (GUID) - ID пользователя.
*   **Ответ:**
    *   `200 OK`: Объект `UserDTO`.
    *   `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `GET /api/v1/User/all`

Получает список всех пользователей.

*   **Аутентификация:** Требуется.
*   **Запрос:** Нет.
*   **Ответ:**
    *   `200 OK`: Массив объектов `UserDTO`.
    *   `500 Internal Server Error`: Сообщение об ошибке (строка).

---

### Профили (`/Profile`)

#### `GET /api/v1/Profile/current-user-profile`

Получает профиль аутентифицированного пользователя.

*   **Аутентификация:** Требуется.
*   **Запрос:** Нет.
*   **Ответ:**
    *   `200 OK`: Объект `ProfileDTO`.
    *   `400 Bad Request`, `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `GET /api/v1/Profile/{userId}`

Получает профиль пользователя по ID.

*   **Аутентификация:** Не требуется.
*   **Параметры пути:**
    *   `userId`: `string` (GUID) - ID пользователя.
*   **Ответ:**
    *   `200 OK`: Объект `ProfileDTO`.
    *   `400 Bad Request`, `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `PATCH /api/v1/Profile/update`

Обновляет профиль аутентифицированного пользователя.

*   **Аутентификация:** Требуется.
*   **Тело запроса:**
    ```typescript
    interface UpdateProfileRequest {
      newStatus?: string;
      newAbout?: string;
    }
    ```
*   **Ответ:**
    *   `200 OK`: Обновленный объект `ProfileDTO`.
    *   `400 Bad Request`, `500 Internal Server Error`: Сообщение об ошибке (строка).

---

### Друзья (`/friends`)

#### `GET /api/v1/friends`

Получает список друзей аутентифицированного пользователя с пагинацией.

*   **Аутентификация:** Требуется.
*   **Параметры запроса:**
    *   `page`: `number` (необязательно, по умолчанию: 1)
    *   `pageSize`: `number` (необязательно, по умолчанию: 10, макс: 100)
*   **Ответ:**
    *   `200 OK`: Массив объектов `FriendshipDTO`.
    *   `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `GET /api/v1/friends/{userId}`

Получает список друзей пользователя по ID с пагинацией.

*   **Аутентификация:** Не требуется.
*   **Параметры пути:**
    *   `userId`: `string` (GUID) - ID пользователя.
*   **Параметры запроса:**
    *   `page`: `number` (необязательно, по умолчанию: 1)
    *   `pageSize`: `number` (необязательно, по умолчанию: 10, макс: 100)
*   **Ответ:**
    *   `200 OK`: Массив объектов `FriendshipDTO`.
    *   `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `GET /api/v1/friends/incoming`

Получает входящие запросы на дружбу для аутентифицированного пользователя с пагинацией.

*   **Аутентификация:** Требуется.
*   **Параметры запроса:**
    *   `page`: `number` (необязательно, по умолчанию: 1)
    *   `pageSize`: `number` (необязательно, по умолчанию: 10, макс: 100)
*   **Ответ:**
    *   `200 OK`: Массив объектов `FriendshipDTO`.
    *   `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `GET /api/v1/friends/outgoing`

Получает исходящие запросы на дружбу для аутентифицированного пользователя с пагинацией.

*   **Аутентификация:** Требуется.
*   **Параметры запроса:**
    *   `page`: `number` (необязательно, по умолчанию: 1)
    *   `pageSize`: `number` (необязательно, по умолчанию: 10, макс: 100)
*   **Ответ:**
    *   `200 OK`: Массив объектов `FriendshipDTO`.
    *   `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `GET /api/v1/friends/possible`

Получает список возможных друзей для аутентифицированного пользователя с пагинацией.

*   **Аутентификация:** Требуется.
*   **Параметры запроса:**
    *   `page`: `number` (необязательно, по умолчанию: 1)
    *   `pageSize`: `number` (необязательно, по умолчанию: 10, макс: 100)
*   **Ответ:**
    *   `200 OK`: Массив объектов `UserListItemDTO`.
    *   `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `POST /api/v1/friends/{friendId}`

Отправляет запрос на дружбу другому пользователю.

*   **Аутентификация:** Требуется.
*   **Параметры пути:**
    *   `friendId`: `string` (GUID) - ID пользователя, которому отправляется запрос.
*   **Ответ:**
    *   `200 OK`: Объект `FriendshipDTO`.
    *   `400 Bad Request`, `409 Conflict`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `PATCH /api/v1/friends/status/{friendshipId}`

Обновляет статус дружбы.

*   **Аутентификация:** Требуется.
*   **Параметры пути:**
    *   `friendshipId`: `string` (GUID) - ID дружбы.
*   **Тело запроса:**
    ```typescript
    interface UpdateFriendshipStatusRequest {
      status: FriendshipStatus;
    }
    ```
*   **Ответ:**
    *   `200 OK`: Обновленный объект `FriendshipDTO`.
    *   `400 Bad Request`, `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

---

### Метки дружбы (`/friends/labels`)

#### `GET /api/v1/friends/labels`

Получает все метки дружбы для аутентифицированного пользователя.

*   **Аутентификация:** Требуется.
*   **Запрос:** Нет.
*   **Ответ:**
    *   `200 OK`: Массив объектов `FriendshipLabelDTO`.
    *   `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `POST /api/v1/friends/labels`

Создает новую метку дружбы.

*   **Аутентификация:** Требуется.
*   **Тело запроса:**
    ```typescript
    interface CreateFriendshipLabelRequest {
      friendshipId: string; // GUID
      label: string;
    }
    ```
*   **Ответ:**
    *   `200 OK`: Объект `FriendshipLabelDTO`.
    *   `400 Bad Request`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `DELETE /api/v1/friends/labels/{friendshipLabelId}`

Удаляет метку дружбы по ID.

*   **Аутентификация:** Требуется.
*   **Параметры пути:**
    *   `friendshipLabelId`: `string` (GUID) - ID метки дружбы.
*   **Ответ:**
    *   `200 OK`.
    *   `400 Bad Request`, `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `DELETE /api/v1/friends/labels?label={label}`

Удаляет все метки дружбы с указанным значением.

*   **Аутентификация:** Требуется.
*   **Параметры запроса:**
    *   `label`: `string` - значение метки.
*   **Ответ:**
    *   `200 OK`.
    *   `400 Bad Request`, `500 Internal Server Error`: Сообщение об ошибке (строка).

---

### Чаты (`/Chats`)

#### `GET /api/v1/Chats`

Получает список всех чатов для аутентифицированного пользователя.

*   **Аутентификация:** Требуется.
*   **Запрос:** Нет.
*   **Ответ:**
    *   `200 OK`: Массив объектов `ChatDTO`.
    *   `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `POST /api/v1/Chats/group`

Создает новый групповой чат.

*   **Аутентификация:** Требуется.
*   **Тело запроса:**
    ```typescript
    interface CreateGroupChatRequest {
      name: string;
      members: UserDTO[];
    }
    ```
*   **Ответ:**
    *   `200 OK`: Объект `ChatDTO`.
    *   `400 Bad Request`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `PATCH /api/v1/Chats/{chatId}`

Обновляет существующий чат.

*   **Аутентификация:** Требуется.
*   **Параметры пути:**
    *   `chatId`: `string` (GUID) - ID чата.
*   **Тело запроса:**
    ```typescript
    interface UpdateChatRequest {
      newName?: string;
    }
    ```
*   **Ответ:**
    *   `200 OK`: Обновленный объект `ChatDTO`.
    *   `400 Bad Request`, `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `DELETE /api/v1/Chats/{chatId}`

Удаляет чат.

*   **Аутентификация:** Требуется.
*   **Параметры пути:**
    *   `chatId`: `string` (GUID) - ID чата.
*   **Ответ:**
    *   `200 OK`.
    *   `400 Bad Request`, `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

---

### Участники чата (`/chats/members`)

#### `GET /api/v1/chats/members/{chatId}`

Получает список участников для определенного чата.

*   **Аутентификация:** Требуется.
*   **Параметры пути:**
    *   `chatId`: `string` (GUID) - ID чата.
*   **Ответ:**
    *   `200 OK`: Массив объектов `ChatMemberDTO`.
    *   `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `POST /api/v1/chats/members`

Добавляет нового участника в чат.

*   **Аутентификация:** Требуется.
*   **Тело запроса:**
    ```typescript
    interface CreateChatMemberRequest {
      chatId: string; // GUID
      memberId: string; // GUID
      inviterId: string; // GUID
    }
    ```
*   **Ответ:**
    *   `200 OK`: Объект `ChatMemberDTO`.
    *   `400 Bad Request`, `403 Forbidden`, `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `PATCH /api/v1/chats/members`

Обновляет роль участника чата.

*   **Аутентификация:** Требуется.
*   **Тело запроса:**
    ```typescript
    interface UpdateChatMemberRequest {
      chatId: string; // GUID
      memberId: string; // GUID
      newMemberName?: string;
      newRole?: ChatMemberRole;
    }
    ```
*   **Ответ:**
    *   `200 OK`: Обновленный объект `ChatMemberDTO`.
    *   `400 Bad Request`, `403 Forbidden`, `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `DELETE /api/v1/chats/members`

Удаляет участника из чата.

*   **Аутентификация:** Требуется.
*   **Тело запроса:**
    ```typescript
    interface DeleteChatMemberRequest {
      chatId: string; // GUID
      memberId: string; // GUID
    }
    ```
*   **Ответ:**
    *   `200 OK`.
    *   `400 Bad Request`, `403 Forbidden`, `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

---

### Сообщения чата (`/chats/messages`)

#### `GET /api/v1/chats/messages/{chatId}`

Получает сообщения для определенного чата, с опциональной фильтрацией по закрепленным сообщениям и пагинацией.

*   **Аутентификация:** Требуется.
*   **Параметры пути:**
    *   `chatId`: `string` (GUID) - ID чата.
*   **Параметры запроса:**
    *   `pinned`: `boolean` (необязательно, по умолчанию: `false`) - если `true`, извлекает только закрепленные сообщения.
    *   `page`: `number` (необязательно, по умолчанию: 1)
    *   `pageSize`: `number` (необязательно, по умолчанию: 10)
*   **Ответ:**
    *   `200 OK`: Массив объектов `ChatMessageDTO`.
    *   `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `POST /api/v1/chats/messages`

Создает новое сообщение чата.

*   **Аутентификация:** Требуется.
*   **Тело запроса:**
    ```typescript
    interface CreateChatMessageRequest {
      chatId: string; // GUID
      message: string;
    }
    ```
*   **Ответ:**
    *   `200 OK`: Объект `ChatMessageDTO`.
    *   `400 Bad Request`, `403 Forbidden`, `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `PATCH /api/v1/chats/messages/edit`

Редактирует существующее сообщение чата.

*   **Аутентификация:** Требуется.
*   **Тело запроса:**
    ```typescript
    interface UpdateEditChatMessageRequest {
      messageId: string; // GUID
      chatId: string; // GUID
      newMessage: string;
    }
    ```
*   **Ответ:**
    *   `200 OK`: Обновленный объект `ChatMessageDTO`.
    *   `400 Bad Request`, `403 Forbidden`, `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `PATCH /api/v1/chats/messages/pin`

Закрепляет или открепляет сообщение чата.

*   **Аутентификация:** Требуется.
*   **Тело запроса:**
    ```typescript
    interface UpdatePinChatMessageRequest {
      messageId: string; // GUID
      chatId: string; // GUID
      isPinned: boolean;
    }
    ```
*   **Ответ:**
    *   `200 OK`: Обновленный объект `ChatMessageDTO`.
    *   `400 Bad Request`, `403 Forbidden`, `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `DELETE /api/v1/chats/messages`

Удаляет сообщение чата.

*   **Аутентификация:** Требуется.
*   **Тело запроса:**
    ```typescript
    interface DeleteChatMessageRequest {
      messageId: string; // GUID
      chatId: string; // GUID
    }
    ```
*   **Ответ:**
    *   `200 OK`.
    *   `400 Bad Request`, `403 Forbidden`, `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

---

### Изображения чата (`/chat-image`)

#### `GET /api/v1/chat-image/{chatId}`

Получает изображение для определенного чата.

*   **Аутентификация:** Требуется.
*   **Параметры пути:**
    *   `chatId`: `string` (GUID) - ID чата.
*   **Ответ:**
    *   `200 OK`: Файл изображения (`image/webp`).
    *   `404 Not Found`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `POST /api/v1/chat-image/{chatId}`

Устанавливает (загружает) изображение для определенного чата.

*   **Аутентификация:** Требуется.
*   **Параметры пути:**
    *   `chatId`: `string` (GUID) - ID чата.
*   **Тело запроса:** `multipart/form-data` с полем `avatarFile` (файл изображения).
    *   Максимальный размер файла: 5 МБ.
    *   Разрешенные расширения: `.jpg`, `.jpeg`, `.png`, `.gif`, `.bmp`.
*   **Ответ:**
    *   `200 OK`: Файл изображения (`image/webp`), представляющий загруженное изображение.
    *   `400 Bad Request`, `500 Internal Server Error`: Сообщение об ошибке (строка).

#### `DELETE /api/v1/chat-image/{chatId}`

Удаляет изображение для определенного чата.

*   **Аутентификация:** Требуется.
*   **Параметры пути:**
    *   `chatId`: `string` (GUID) - ID чата.
*   **Ответ:**
    *   `200 OK`.
    *   `500 Internal Server Error`: Сообщение об ошибке (строка).
