import { Controller, Get } from '@nestjs/common';
import { AppService } from './app.service';
import { AppInfo } from './types';

@Controller()
export class AppController {
  constructor(private readonly appService: AppService) { }

  @Get()
  getHello(): AppInfo {
    return this.appService.getInfo();
  }
}
