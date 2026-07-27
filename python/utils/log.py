# class Log:
#     @staticmethod
#     def red(mssg: str):
#         print(f"\x1b[31m{mssg}\x1b[0m")
#
#     @staticmethod
#     def green(mssg: str):
#         print(f"\x1b[32m{mssg}\x1b[0m")
#
#     @staticmethod
#     def blue(mssg: str):
#         print(f"\x1b[34m{mssg}\x1b[0m")
#
#     @staticmethod
#     def magenta(mssg: str):
#         print(f"\x1b[35m{mssg}\x1b[0m")
#
#     @staticmethod
#     def cyan(mssg: str):
#         print(f"\x1b[36m{mssg}\x1b[0m")
#
#     @staticmethod
#     def teal(mssg: str):
#         print(f"\x1b[33m{mssg}\x1b[0m")


import logging
import sys


class Log:
    # 1. Configuramos el logger
    logger = logging.getLogger("SolutionLogger")
    logger.setLevel(logging.INFO)

    # 2. Evitamos que se dupliquen los logs si ya existe un handler
    if not logger.handlers:
        handler = logging.StreamHandler(sys.stdout)
        # Formato simple para que no ensucie la lógica de colores
        formatter = logging.Formatter("%(message)s")
        handler.setFormatter(formatter)
        logger.addHandler(handler)

    @staticmethod
    def _log(mssg: str, color_code: str):
        # Usamos el logger en lugar de print
        Log.logger.info(f"{color_code}{mssg}\x1b[0m")

    @staticmethod
    def red(mssg: str):
        Log._log(mssg, "\x1b[31m")

    @staticmethod
    def green(mssg: str):
        Log._log(mssg, "\x1b[32m")

    @staticmethod
    def blue(mssg: str):
        Log._log(mssg, "\x1b[34m")

    @staticmethod
    def magenta(mssg: str):
        Log._log(mssg, "\x1b[35m")

    @staticmethod
    def cyan(mssg: str):
        Log._log(mssg, "\x1b[36m")

    @staticmethod
    def teal(mssg: str):
        Log._log(mssg, "\x1b[33m")
