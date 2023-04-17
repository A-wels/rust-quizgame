function getHostIp () {
    var url =  window.location.href
    var url = url.split("//")[1]
    var ip = url.split(":")[0]
    return ip
  }