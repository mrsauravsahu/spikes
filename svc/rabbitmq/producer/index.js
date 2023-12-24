#!/usr/bin/env node

var amqp = require('amqplib/callback_api');

amqp.connect(process.env.AMQP_CONNECTION_STRING, function (err, connection) {
    if (err) {
        throw err;
    }

    connection.createChannel(function (channelCreationError, channel) {
        if (channelCreationError) {
            throw channelCreationError;
        }

        var queue = process.env.AMQP_QUEUE || 'hello';
        var message = process.env.MESSAGE || 'Hello world';

        channel.assertQueue(queue, {
            durable: true
        });

        channel.sendToQueue(queue, Buffer.from(message));
        console.log("[x] Sent %s", message);

        setTimeout(() => {
            connection.close();
            process.exit(0);
        }, 100);
    });
});