import { useEffect, useState } from 'react';

const useWebSocket = (url: string) => {
    const [data, setData] = useState<any>(null);

    useEffect(() => {
        const socket = new WebSocket(url);

        socket.onmessage = (event) => {
            const messageData = JSON.parse(event.data);
            setData(messageData);
        };

        return () => {
            socket.close();
        };
    }, [url]);

    return data;
};

export default useWebSocket;
