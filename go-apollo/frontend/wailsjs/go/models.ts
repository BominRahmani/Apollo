export namespace main {
	
	export class Wallpaper {
	    URL: string;
	    Preview: string;
	    Resolution: string;
	
	    static createFrom(source: any = {}) {
	        return new Wallpaper(source);
	    }
	
	    constructor(source: any = {}) {
	        if ('string' === typeof source) source = JSON.parse(source);
	        this.URL = source["URL"];
	        this.Preview = source["Preview"];
	        this.Resolution = source["Resolution"];
	    }
	}

}

