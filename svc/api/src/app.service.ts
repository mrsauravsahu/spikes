import { Injectable } from '@nestjs/common';
import { AppInfo } from './types';

@Injectable()
export class AppService {
  getInfo(): AppInfo {
    const {
      API_SERVICE_NAME: name = 'API Service',
      API_SERVICE_VERSION: version,
    } = process.env;
    return {
      name,
      version,
    };
  }
}
