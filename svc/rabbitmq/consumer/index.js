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

    channel.assertQueue(queue, {
      durable: true
    });

    channel.consume(queue, function (message) {
      console.log("[x] Received %s", JSON.stringify({ ...message, data: undefined, content: undefined}, undefined, 2));
      console.log("[x] Received message: %s", JSON.stringify(message.content.toString(), undefined, 2));

      channel.ack(message)
    });
  });
});