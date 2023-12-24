import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);

  const { PORT = 80 } = process.env;

  await app.listen(PORT);
  console.log(`listening at ${await app.getUrl()}`)
}
bootstrap();
