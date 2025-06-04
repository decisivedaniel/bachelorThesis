#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *getMessage(char *message);
char *retrieveMessage(char *message);
char *runLengthEncode(char *message);
void createWNPattern(char* whiteNoise, char* rowWhiteNoise);
char *encryptMessage(char* message, int messageLength, char* whiteNoise);
void createBMP(char* encryptedWhiteNoise, char *whiteNoise);
void readBMP(char* encryptedWhiteNoise, char *whiteNoise);
char* runLengthDecode(char *encodedMessage);

int main()
{
    char* whiteNoise = calloc(1024, sizeof (char));
    char* rowWhiteNoise = "WWWWWWWWBBBBBBBBWWWWBBBBWWBBBWWR";
    char* message = "Reading Dilbert strips or encoding Elbonian messages are not good excuses for failing the XBC009 final exam.";

    printf("enter 0 for encryption, enter 2 for decryption\n");
    int input = 0;
    scanf("%d", &input);

    // Repeating White Noise Pattern
    createWNPattern(whiteNoise, rowWhiteNoise);

    if (input == 0)
    {
        char* encryptedWhiteNoise;

        // Message
        message = getMessage(message);

        // Run Length Encoding
        message = runLengthEncode(message);

        // Encryption
        //encryptedWhiteNoise = encryptMessage(message, strlen(message), whiteNoise);

        // BMP 
        createBMP(message, whiteNoise);


        free(message);
        //free(encryptedWhiteNoise);
        
    }
    else
    {
        char* encryptedWhiteNoise = calloc(1024, sizeof(char));
        //char* unencryptedMessage = calloc(1024, sizeof(char));

        readBMP(encryptedWhiteNoise, whiteNoise);

        //unencryptedMessage = encryptMessage(encryptedWhiteNoise, 1024, whiteNoise);

        encryptedWhiteNoise = runLengthDecode(encryptedWhiteNoise);

        encryptedWhiteNoise = retrieveMessage(encryptedWhiteNoise);
        printf("%s\n", encryptedWhiteNoise);

        //free(unencryptedMessage);
        free(encryptedWhiteNoise);
    }

    free(whiteNoise);
    
}

char* getMessage(char* message)
{
    char *lead = "XXXXXXXXBBBBCCCCOOOO000099999999";
    char *new = malloc(200);
    strcat(new, lead);
    strcat(new, message);
    strcat(new, lead);
    return new;
}


char *retrieveMessage(char *message)
{
    char *new = malloc(200);
    int messageLength = strlen(message) - 32;
    int i = 32;
    while (i < messageLength)
    {
        new[i-32] = message[i];
        i++;
    }
    return new;
}

char* runLengthEncode(char* message)
{
    char *encoded = malloc(1024);
    int i = 0;
    int j = 0;
    int length = 0;
    char current = message[i];
    char last = message[i];
    while (last != 0x00)
    {
        if(current == last)
        {
            length++;
        }
        else
        {
            encoded[j] = length+48;
            j++;
            encoded[j] = last;
            j++;
            length = 1;
        }
        i++;
        last = current;
        current = message[i];
    };

    return encoded;

}

char* runLengthDecode(char* encodedMessage)
{
    char* decoded = calloc(4048, sizeof(char));
    int amount = 0;
    char current;
    int encodedI = 0;
    int decodedI = 0;
    int encodedMessageLength = strlen(encodedMessage);
    if (encodedMessageLength > 1024)
    {
        encodedMessageLength = 1024;
    }
    while(decodedI < encodedMessageLength)
    {
        amount = encodedMessage[encodedI] - 48;
        encodedI++;
        current = encodedMessage[encodedI];
        encodedI++;
        if (current == 0x00)
        {
            decodedI = encodedMessageLength;
        }
        int j = 0;
        while(j < amount)
        {
            decoded[decodedI] = current;
            j++;
            decodedI++;
        }
    }
    return decoded;
}

void createWNPattern(char* whiteNoise, char* rowWhiteNoise)
{
    //32 rows
    int i = 0;
    while (i < 32)
    {
        //32 columns
        int j = 0;
        while (j < 32)
        {
            whiteNoise[(i*32) + j] = rowWhiteNoise[j];
            j++;
        }
        i++;
    }

}


char* encryptMessage(char* message, int messageLength, char* whiteNoise)
{
    char current = message[0];
    int whiteNoiseLength = strlen(whiteNoise);
    char* encryptMessage = malloc(1024);
    int max = whiteNoiseLength;
    int i = 0;
    while (i < max)
    {
        encryptMessage[i%whiteNoiseLength] = whiteNoise[i%whiteNoiseLength]^current;
        i++;
        current = message[i%messageLength];
    }
    return encryptMessage;
}

void createBMP(char* encryptedWhiteNoise, char* whiteNoise)
{
    // file open
    FILE* bmp = fopen("output.bmp", "w");

    // file header
    // signature "BM"
    char *signiture = "BM";
    fwrite(signiture, sizeof(char),2,bmp);

    // file size 14 + 40 + 1024 = 1078 or 010000110110
    // 0000 0100 0011 0110 or 0x0436
    unsigned char fileSize[] = {0x36, 0x04, 0x00, 0x00};
    fwrite(signiture, sizeof(char), 4, bmp);


    // reserved field (four bytes hex)
    // 00 00 00 00
    unsigned char reserved[] = {0x00, 0x00, 0x00, 0x00}; 
    fwrite(reserved, sizeof(char), 4, bmp);

    // offset pixel data (four bytes int)
    // 54 or 36 00 00 00
    unsigned char offset[] = {0x36, 0x00, 0x00, 0x00};
    fwrite(offset, sizeof(char), 4, bmp);


    // Bitmap Header
    // header: 40 or 28 00 00 00
    // width: 32 or 20 00 00 00
    // height: 32 or 20 00 00 00
    // reserved: 01 00
    // bits per pixel: 24 or 18 00
    // compression: 00 00 00 00
    // size of pixel data: 1024 or 00 04 00 00
    // horizontal resolution: 2835 or 13 0B 00 00
    // vertical resolution: 2835 or 13 0B 00 00
    // color palette: 00 00 00 00
    // important colors: 00 00 00 00
    unsigned char header[] = {0x28, 0x00, 0x00, 0x00,
                    0x20, 0x00, 0x00, 0x00,
                    0x20, 0x00, 0x00, 0x00,
                    0x01, 0x00,
                    0x18, 0x00,
                    0x00, 0x00, 0x00, 0x00,
                    0x00, 0x04, 0x00, 0x00,
                    0x13, 0x0B, 0x00, 0x00,
                    0x13, 0x0B, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x00};
    fwrite(header, sizeof(char), 40, bmp);

    // Pixel Data
    int eWNLength = strlen(encryptedWhiteNoise);
    unsigned char* currentValue = malloc(3);
    for (int i = 0; i < 1024; i++)
    {
        char currentWhiteNoise = whiteNoise[i];
        if (currentWhiteNoise == 'W')
        {
            currentValue[0] = 0xff;
            currentValue[1] = 0xff;
        }
        else 
        {
            currentValue[0] = 0x00;
            currentValue[1] = 0x00;
        }
        if (currentWhiteNoise == 'B')
        {
            currentValue[2] = 0x00;
        }
        else 
        {
            currentValue[2] = 0xff;
        }
        if (i < eWNLength)
        {
            //encode message into white noise
            unsigned char encryptedChar = encryptedWhiteNoise[i];
            int divide = encryptedChar / 16;
            int mod = encryptedChar % 16;
            int flag = 0;
            if (divide >= 8)
            {
                divide -= 8;
                flag += 2;
            }
            if (mod >= 8)
            {
                mod -= 8;
                flag += 1;
            }
            currentValue[0] ^= divide;
            currentValue[1] ^= mod;
            currentValue[2] ^= flag;
        }
        
        fwrite(currentValue, 1, 3, bmp);
    }
    free(currentValue);
    // file close
    fclose(bmp);
}

void readBMP(char* encryptedWhiteNoise, char *whiteNoise)
{
    FILE* bmp = fopen("output.bmp", "r");

    fread(encryptedWhiteNoise, sizeof(char), 54, bmp);

    unsigned char *pixel = calloc(3, sizeof(char));
    for (int i=0; i < 1024; i++)
    {
        int divide, mod, flag;
        unsigned char currentWhiteNoise = whiteNoise[i];
        fread(pixel, sizeof(char), 3, bmp);
        if (currentWhiteNoise == 'W')
        {
            divide = 0xff;
            mod = 0xff;
        }
        else
        {
            divide = 0x00;
            mod = 0x00;
        }
        if (currentWhiteNoise == 'B')
        {
            flag = 0x00;
        }
        else
        {
            flag = 0xff;
        }
        divide ^= pixel[0];
        mod ^= pixel[1];
        flag ^= pixel[2];
        if (flag > 1)
        {
            divide += 8;
            flag -= 2;
        }
        if (flag > 0)
        {
            mod += 8;
        }
        divide *= 16;
        divide += mod;
        encryptedWhiteNoise[i] = (unsigned char)divide;

    }

    free(pixel);

    fclose(bmp);
}